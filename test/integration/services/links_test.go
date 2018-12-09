package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/services"
)

func TestUpsertBadLink(t *testing.T) {
	ctx := context.Background()

	result, err := services.UpsertLink(ctx, testDB, orgId, "topic name", nil, []string{})
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
