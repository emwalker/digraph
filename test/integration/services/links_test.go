package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/internal/models"
	"github.com/emwalker/digraph/internal/services"
)

func TestUpsertBadLink(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}

	result, err := c.UpsertLink(context.Background(), defaultRepo, "topic name", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if len(result.Alerts) < 1 {
		t.Fatal("Expected one or more alerts")
	}

	if result.Link != nil {
		t.Fatal("A link should not have been created")
	}
}

func TestLinkHasATopic(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	title := "A title"
	result, err := c.UpsertLink(ctx, defaultRepo, "http://some.url.com/", &title, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if !result.LinkCreated {
		t.Fatal("Expected link to be a new one")
	}

	topics, err := result.Link.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		t.Fatal(err)
	}

	if len(topics) < 1 {
		t.Fatal("Expected the link to be added to the root topic")
	}
}

func TestUserLinkHistory(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	prevCount, _ := models.UserLinks().Count(ctx, testDB)
	var nextCount int64

	// A log is added for an upsert
	title := "A title"
	upsertResult, err := c.UpsertLink(ctx, defaultRepo, "http://frotz.com/", &title, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer upsertResult.Cleanup()

	nextCount, _ = models.UserLinks().Count(ctx, testDB)
	if (prevCount + 1) != nextCount {
		t.Fatal("Expected a new user link record to be created for the upsert")
	}

	// A log is added for a delete
	deleteResult, err := c.DeleteLink(ctx, defaultRepo, upsertResult.Link)
	if err != nil {
		t.Fatal(err)
	}
	defer deleteResult.Cleanup()

	nextCount, _ = models.UserLinks().Count(ctx, testDB)
	if (prevCount + 2) != nextCount {
		t.Fatal("Expected a new user link record to be created for the delete")
	}
}
