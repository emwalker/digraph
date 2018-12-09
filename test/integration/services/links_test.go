package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/services"
)

func TestUpsertBadLink(t *testing.T) {
	c := services.Connection{
		Exec:  testDB,
		Actor: testActor,
	}

	result, err := c.UpsertLink(context.Background(), defaultRepo, "topic name", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}

	if len(result.Alerts) < 1 {
		t.Fatal("Expected one or more alerts")
	}

	if result.Link != nil {
		t.Fatal("A link should not have been created")
	}
}
