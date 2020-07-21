package queries_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/queries"
	in "github.com/emwalker/digraph/test/integration"
)

func TestLinksForReviewIncludesDescendantLinks(t *testing.T) {
	mutator := in.NewMutator(in.MutatorOptions{})
	parentTopic := mutator.UpsertTopic(in.UpsertTopicOptions{Name: "A topic"})
	childTopic := mutator.UpsertTopic(in.UpsertTopicOptions{
		Name:           "Child topic",
		ParentTopicIds: []string{parentTopic.ID},
	})

	mutator.UpsertLink(in.UpsertLinkOptions{
		URL:            "http://a-url.org",
		ParentTopicIds: []string{childTopic.ID},
	})

	first := 100
	reviewed := false

	query := queries.LinkQuery{
		First:              &first,
		IncludeDescendants: true,
		Reviewed:           &reviewed,
		SearchString:       nil,
		Topic:              parentTopic.Topic,
		View:               in.View,
		Viewer:             in.Actor,
	}

	links, err := query.Fetch(context.Background(), in.DB)
	in.Must(err)

	if len(links) < 1 {
		t.Fatal("Expected a link")
	}
}
