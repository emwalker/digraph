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

// TopicTimeline returns the timeline for a topic.
func TopicTimeline(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic,
) (*models.TopicTimeline, error) {
	if topic.R != nil {
		if len(topic.R.TopicTimelines) > 0 {
			return topic.R.TopicTimelines[0], nil
		}
		return nil, nil
	}

	log.Printf("Fetching topic timeline")
	timeline, err := topic.TopicTimelines().One(ctx, exec)
	if err != nil {
		if err.Error() == ErrSQLNoRows {
			return nil, nil
		}
		return nil, errors.Wrap(err, "resolvers: failed to fetch timeline")
	}
	return timeline, err
}
