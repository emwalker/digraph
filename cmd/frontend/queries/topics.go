package queries

import (
	"context"
	"fmt"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/redis"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// Topic looks for a topic within the topics that are visible to the current user.
func Topic(actorID, topicID string) []qm.QueryMod {
	return []qm.QueryMod{
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.InnerJoin("organization_members om on r.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", topicID, actorID),
	}
}

// TimeRange returns the time range for a topic.
func TimeRange(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic,
) (*models.Timerange, error) {
	if topic.R != nil && topic.R.Timerange != nil {
		return topic.R.Timerange, nil
	}

	log.Printf("Fetching topic time range")
	timerange, err := topic.Timerange().One(ctx, exec)
	if err != nil {
		if err.Error() == ErrSQLNoRows {
			return nil, nil
		}
		return nil, errors.Wrap(err, "resolvers: failed to fetch timerange")
	}
	return timerange, err
}

// MatchingDescendantTopicIds uses the topic and search string to find the transitive closure of matching
// descendant topics.
func MatchingDescendantTopicIds(
	ctx context.Context, conn redis.Connection, exec boil.ContextExecutor, topic *models.TopicValue,
	searchString string, limit int,
) ([]interface{}, error) {
	q := NewSearchQuery(searchString)

	rows := []struct {
		ID string
	}{}

	log.Printf("Looking for topics under %s with query: %s", topic, searchString)

	sql := fmt.Sprintf(`
	with recursive child_topics as (
		select parent_id, parent_id as child_id
		from topic_topics where parent_id = $1
	union
		select pt.child_id, ct.child_id
		from topic_topics ct
		inner join child_topics pt on pt.child_id = ct.parent_id
	)
	select distinct t.id
	from topics t
	inner join child_topics ct on ct.child_id = t.id
	where (
		case $2
		when '' then true
		else to_tsvector('synonymsdict', t.synonyms) @@ to_tsquery('synonymsdict', %s)
		end
	)
	limit $3
	`, q.PostgresTsQueryInput())

	err := queries.Raw(sql, topic.ID, searchString, limit).Bind(ctx, exec, &rows)

	if err != nil {
		log.Printf("There was a problem with this sql: %s", sql)
		return nil, errors.Wrap(err, "resolvers: failed to fetch descendant topics")
	}

	var topicIds []interface{}
	for _, row := range rows {
		topicIds = append(topicIds, row.ID)
	}

	return topicIds, nil
}
