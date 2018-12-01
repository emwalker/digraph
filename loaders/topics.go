package loaders

//go:generate dataloaden -keys string github.com/emwalker/digraph/models.Topic

import (
	"context"
	"database/sql"
	"log"
	"time"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

const TopicLoaderKey = "topicLoader"

type config struct {
	db *sql.DB
}

func convertIds(ids []string) []interface{} {
	var translatedIds []interface{}
	for _, id := range ids {
		translatedIds = append(translatedIds, id)
	}
	return translatedIds
}

func fetchTopicsFromDB(ctx context.Context, c *config) func(ids []string) ([]*models.Topic, []error) {
	return func(ids []string) ([]*models.Topic, []error) {
		log.Print("Fetching topic ids", ids)
		topics, err := models.Topics(
			qm.WhereIn("id in ?", convertIds(ids)...),
			qm.Load("ParentTopics"),
		).All(ctx, c.db)

		if err != nil {
			return nil, []error{err}
		}
		return topics, nil
	}
}

func NewTopicLoader(ctx context.Context, db *sql.DB) *TopicLoader {
	return &TopicLoader{
		maxBatch: 100,
		wait:     1 * time.Millisecond,
		fetch:    fetchTopicsFromDB(ctx, &config{db}),
	}
}
