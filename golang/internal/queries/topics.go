package queries

import (
	"context"
	"log"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// Fetch a topic together with any relations that should be loaded
func FetchTopic(
	ctx context.Context, exec boil.ContextExecutor, topicID string, actor *models.User,
) (*models.Topic, error) {
	topic, err := models.Topics(
		qm.Load("Repository"),
		qm.Load("Repository.Owner"),
		qm.InnerJoin("organization_members om on topics.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", topicID, actor.ID),
	).One(ctx, exec)
	return topic, err
}

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

	log.Printf("queries: fetching topic time range")
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
	log.Printf("queries: fetching parent topics for topic %s", q.Topic)
	mods := q.Filter([]qm.QueryMod{
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.OrderBy("topics.name"),
	})

	return q.Topic.ParentTopics(mods...).All(ctx, exec)
}
