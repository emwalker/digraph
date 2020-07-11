package queries

import (
	"context"
	"fmt"
	"log"
	"strings"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries/parser"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
	"github.com/volatiletech/sqlboiler/v4/types"
)

// SearchSpec fetches the intersection of the transitive closures of the parent topic ids provided so that
// further searches to can be carried out.
type SearchSpec struct {
	*parser.QuerySpec
	searchString *string
	parentTopic  *models.TopicValue
}

// Search helps with the fetching of child topics and links given the transitive closure of topic ids
// that have been provided.
type Search struct {
	*SearchSpec
	TopicTransitiveClosure []interface{}
}

// StartingTopicIds returns all of the parent topic ids that will be used in the search
func (s SearchSpec) StartingTopicIds() []interface{} {
	var ids []interface{}
	ids = append(ids, s.parentTopic.ID)
	for _, topic := range s.Topics {
		ids = append(ids, topic.ID())
	}
	return ids
}

func (s SearchSpec) queryParameters() []interface{} {
	var parameters []interface{}
	for _, topicID := range s.StartingTopicIds() {
		parameters = append(parameters, topicID)
	}
	return parameters
}

func (s SearchSpec) toString() string {
	var buffer []string

	topicIds := s.StartingTopicIds()

	buffer = append(buffer, "with recursive")
	var commonTableExpressions []string
	for idx := range topicIds {
		pos := idx + 1
		cte := fmt.Sprintf(`
	child_topics%d as (
		select parent_id, parent_id as child_id
		from topic_topics where parent_id = $%d
	union
		select pt.child_id, ct.child_id
		from topic_topics ct
		inner join child_topics%d pt on pt.child_id = ct.parent_id
	)`, pos, pos, pos)
		commonTableExpressions = append(commonTableExpressions, cte)
	}
	buffer = append(buffer, strings.Join(commonTableExpressions, ", "))

	buffer = append(buffer, `
	select distinct t.id
	from topics t
	`)

	for idx := range topicIds {
		pos := idx + 1
		buffer = append(buffer, fmt.Sprintf(`
	inner join child_topics%d ct%d on ct%d.child_id = t.id
		`, pos, pos, pos))
	}

	sql := strings.Join(buffer, "")
	return sql
}

// NewSearch returns a SearchSpec that can then be used to search for topics and links within a set of parent
// topics.
func NewSearch(parentTopic *models.TopicValue, searchString *string) *SearchSpec {
	querySpec := parser.Parse(searchString)
	return &SearchSpec{
		QuerySpec:    querySpec,
		parentTopic:  parentTopic,
		searchString: searchString,
	}
}

// Exec fetches the transitive closure of topic ids and then returns a Search instance that can be used
// for further queries.
func (s *SearchSpec) Exec(ctx context.Context, exec boil.ContextExecutor) (*Search, error) {
	rows := []struct {
		ID string
	}{}

	query := queries.Raw(s.toString(), s.queryParameters()...)
	if err := query.Bind(ctx, exec, &rows); realError(err) {
		log.Printf("There was a problem with the sql: %s, using params %s and sql: %s", err, s.queryParameters(), s.toString())
		return nil, errors.Wrap(err, "resolvers: failed to fetch descendant topics")
	}

	var topicIds []interface{}
	for _, row := range rows {
		topicIds = append(topicIds, row.ID)
	}

	return &Search{
		SearchSpec:             s,
		TopicTransitiveClosure: topicIds,
	}, nil
}

// DescendantTopics returns subtopics within matching topics that match the search terms provided.
func (s Search) DescendantTopics(
	ctx context.Context, exec boil.ContextExecutor, limit int,
) ([]*models.Topic, error) {
	if len(s.TopicTransitiveClosure) < 1 {
		var topics []*models.Topic
		return topics, nil
	}

	tokenInput := s.TokenInput()
	var ids []interface{}

	if tokenInput == "" {
		for _, topicID := range s.TopicTransitiveClosure {
			ids = append(ids, topicID)
		}
		log.Printf("matching topic ids (no string match): %v", ids)
	} else {
		rows := []struct {
			ID string
		}{}

		sql := fmt.Sprintf(`
		select t.id
		from topics t
		where t.id = any($3)
		and (
			case $1
			when '' then true
			else (
				to_tsvector('synonymsdict', t.synonyms) @@ to_tsquery('synonymsdict', %s)
			)
			end
		)
		limit $2
		`, s.EscapedPostgresTsQueryInput())

		err := queries.Raw(
			sql,
			tokenInput,
			limit,
			types.Array(s.TopicTransitiveClosure),
		).Bind(ctx, exec, &rows)
		if realError(err) {
			return nil, errors.Wrap(err, "resolvers: failed to fetch topics")
		}

		for _, row := range rows {
			ids = append(ids, row.ID)
		}
		log.Printf("matching topic ids (string match): %v, %s", ids, *s.searchString)
	}

	mods := s.parentTopic.View.Filter([]qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.WhereIn("topics.id in ?", ids...),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.OrderBy("char_length(topics.name), topics.name"),
	})

	topics, err := models.Topics(mods...).All(ctx, exec)
	if realError(err) {
		return nil, errors.Wrap(err, "resolvers: failed to fetch topics")
	}
	return topics, nil
}

// DescendantLinks returns links within matching topics that match the search terms provided.
func (s Search) DescendantLinks(
	ctx context.Context, exec boil.ContextExecutor, limit int,
) ([]*models.Link, error) {
	rows := []struct {
		ID string
	}{}

	sql := fmt.Sprintf(`
	select l.id from links l
	inner join link_topics lt on l.id = lt.child_id
	where lt.parent_id = any($4)
	and (
		case $1
		when '' then true
		else (
			to_tsvector('linksdict', l.title) @@ to_tsquery('linksdict', %s)
			or l.url ~~* all($2)
		)
		end
	)
	limit $3
	`, s.EscapedPostgresTsQueryInput())

	err := queries.Raw(
		sql,
		s.TokenInput(),
		s.WildcardStringArray(),
		limit,
		types.Array(s.TopicTransitiveClosure),
	).Bind(ctx, exec, &rows)

	if realError(err) {
		log.Printf("There was a problem with this sql: %s", sql)
		return nil, errors.Wrap(err, "resolvers: failed to fetch descendant links")
	}

	var ids []interface{}
	for _, row := range rows {
		ids = append(ids, row.ID)
	}

	mods := s.parentTopic.View.Filter([]qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.WhereIn("links.id in ?", ids...),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	links, err := models.Links(mods...).All(ctx, exec)
	if realError(err) {
		return nil, err
	}

	return links, nil
}
