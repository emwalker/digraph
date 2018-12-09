package common

import (
	"github.com/emwalker/digraph/models"
)

func TopicsFromIds(topicIds []string) []*models.Topic {
	var topics []*models.Topic
	for _, topicID := range topicIds {
		topics = append(topics, &models.Topic{ID: topicID})
	}
	return topics
}
