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

// TopicTimeRange returns the time range for a topic.
func TopicTimeRange(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic,
) (*models.TopicTimerange, error) {
	if topic.R != nil {
		if len(topic.R.TopicTimeranges) > 0 {
			return topic.R.TopicTimeranges[0], nil
		}
		return nil, nil
	}

	log.Printf("Fetching topic time range")
	timerange, err := topic.TopicTimeranges().One(ctx, exec)
	if err != nil {
		if err.Error() == ErrSQLNoRows {
			return nil, nil
		}
		return nil, errors.Wrap(err, "resolvers: failed to fetch timerange")
	}
	return timerange, err
}
