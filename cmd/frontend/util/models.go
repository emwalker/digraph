package util

import (
	"github.com/emwalker/digraph/cmd/frontend/models"
)

// TopicsFromIds returns a set of topics initialized from the ids provided as input.
func TopicsFromIds(topicIds []string) []*models.Topic {
	var topics []*models.Topic
	for _, topicID := range topicIds {
		topics = append(topics, &models.Topic{ID: topicID})
	}
	return topics
}
