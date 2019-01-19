package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/internal/services"
)

func TestUpsertTopicEnsuresATopic(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "62ce187241e", nil, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer result.Cleanup()

	if result.TopicCreated != true {
		t.Fatal("Expected topic to be a new one")
	}

	topics, err := result.Topic.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		t.Fatalf("Unable to fetch parent topics: %s", err)
	}

	if len(topics) < 1 {
		t.Fatal("Expected the topic to be added to the root topic")
	}
}
