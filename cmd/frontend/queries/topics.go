package queries

import (
	"context"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
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

// TopicParentTopics returns the parent topics of a topic
type TopicParentTopics struct {
	*models.View
	Topic *models.Topic
}

// Fetch fetches the parent topics
func (q TopicParentTopics) Fetch(ctx context.Context, exec boil.ContextExecutor) ([]*models.Topic, error) {
	log.Printf("Fetching parent topics for topic %s", q.Topic)
	mods := q.Filter([]qm.QueryMod{
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.OrderBy("topics.name"),
	})

	return q.Topic.ParentTopics(mods...).All(ctx, exec)
}
