package loaders

//go:generate go run github.com/vektah/dataloaden TopicLoader string "*github.com/emwalker/digraph/golang/internal/models.Topic"

import (
	"context"
	"log"
	"time"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// TopicLoaderKey is the key under which the topic loader is stored in the session.
const TopicLoaderKey = "topicLoader"

type topicFetcher func(ids []string) ([]*models.Topic, []error)

func fetchTopicsFromDB(ctx context.Context, c *config) topicFetcher {
	return func(ids []string) ([]*models.Topic, []error) {
		log.Print("Fetching topic ids", ids)
		topics, err := models.Topics(
			qm.WhereIn("id in ?", convertIds(ids)...),
			qm.Load("ParentTopics"),
		).All(ctx, c.exec)

		if err != nil {
			return nil, []error{err}
		}
		return topics, nil
	}
}

// NewTopicLoader returns a new topic loader.
func newTopicLoader(ctx context.Context, exec boil.ContextExecutor, wait time.Duration) *TopicLoader {
	return NewTopicLoader(TopicLoaderConfig{
		MaxBatch: 100,
		Wait:     wait,
		Fetch:    fetchTopicsFromDB(ctx, &config{exec}),
	})
}
