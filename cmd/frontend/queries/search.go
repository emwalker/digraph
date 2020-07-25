package queries

import (
	"context"
	"fmt"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries/parser"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// Search helps with the fetching of child topics and links
type Search struct {
	*parser.QuerySpec
	searchString *string
	parentTopic  *models.TopicValue
}

// NewSearch returns a Search that can then be used to search for topics and links within a set of parent
// topics.
func NewSearch(parentTopic *models.TopicValue, searchString *string) *Search {
	querySpec := parser.Parse(searchString)
	return &Search{
		QuerySpec:    querySpec,
		parentTopic:  parentTopic,
		searchString: searchString,
	}
}

func (s Search) startingTopicIds() []interface{} {
	var ids []interface{}
	for _, topicID := range s.ExplicitTopicIds() {
		ids = append(ids, topicID)
	}
	ids = append(ids, s.parentTopic.ID)
	return ids
}

// DescendantTopics returns subtopics within matching topics that match the search terms provided.
func (s Search) DescendantTopics(
	ctx context.Context, exec boil.ContextExecutor, limit int,
) ([]*models.Topic, error) {
	var err error
	var topics []*models.Topic

	if limit < 1 {
		return topics, nil
	}

	whereClause := fmt.Sprintf(`
	(
		case ?
		when '' then true
		else (
			to_tsvector('synonymsdict', topics.synonyms) @@ to_tsquery('synonymsdict', %s)
		)
		end
	)
	`, s.EscapedPostgresTsQueryInput())

	mods := s.parentTopic.View.Filter([]qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.Where(whereClause, s.TokenInput()),
		qm.OrderBy("char_length(topics.name), topics.name"),
		qm.Limit(limit),
	})

	for idx, topicID := range s.startingTopicIds() {
		mods = append(
			mods,
			qm.InnerJoin(fmt.Sprintf("topic_transitive_closure ttc%d on topics.id = ttc%d.child_id", idx, idx)),
			qm.Where(fmt.Sprintf("ttc%d.parent_id = ?", idx), topicID),
		)
	}

	topics, err = models.Topics(mods...).All(ctx, exec)
	if IsRealError(err) {
		log.Printf("There was a problem searching topics: %s", err)
		return nil, errors.Wrap(err, "resolvers: failed to fetch topics")
	}
	return topics, nil
}

// DescendantLinks returns links within matching topics that match the search terms provided.
func (s Search) DescendantLinks(
	ctx context.Context, exec boil.ContextExecutor, limit int,
) ([]*models.Link, error) {
	var err error
	var links []*models.Link

	if limit < 1 {
		return links, nil
	}

	whereClause := fmt.Sprintf(`
	(
		case ?
		when '' then true
		else (
			to_tsvector('linksdict', links.title) @@ to_tsquery('linksdict', %s)
			or links.url ~~* all(?)
		)
		end
	)
	`, s.EscapedPostgresTsQueryInput())

	mods := s.parentTopic.View.Filter([]qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
		qm.Where(whereClause, s.TokenInput(), s.WildcardStringArray()),
		qm.Limit(limit),
	})

	for idx, topicID := range s.startingTopicIds() {
		mods = append(
			mods,
			qm.InnerJoin(fmt.Sprintf("link_transitive_closure ltc%d on links.id = ltc%d.child_id", idx, idx)),
			qm.Where(fmt.Sprintf("ltc%d.parent_id = ?", idx), topicID),
		)
	}

	links, err = models.Links(mods...).All(ctx, exec)
	if IsRealError(err) {
		log.Printf("There was a problem: %s", err)
		return nil, errors.Wrap(err, "resolvers: failed to fetch links")
	}

	return links, nil
}
