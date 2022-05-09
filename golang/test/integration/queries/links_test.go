package queries_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/queries"
	in "github.com/emwalker/digraph/golang/test/integration"
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

func TestLinkQueryEagerLoadedParentTopicsAreFiltered(t *testing.T) {
	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteRepositoriesByName("Repo 1", "Repo 2")
	mutator.DeleteTopicsByName("Parent 1", "Parent 2")
	mutator.DeleteLinksByURL("https://example.com/some-url")
	mutator.DeleteUsersByEmail("gnusto@example.com", "frotz@example.com")
	mutator.DeleteOrganizationsByLogin("gnusto", "frotz")

	user1, result := mutator.CreateUser(in.CreateUserOptions{
		Name:  "Gnusto",
		Email: "gnusto@example.com",
		Login: "gnusto",
	})
	repo1 := result.Repository
	parent1 := mutator.UpsertTopic(in.UpsertTopicOptions{Name: "Parent 1", Repository: repo1})

	_, result = mutator.CreateUser(in.CreateUserOptions{
		Name:  "Frotz",
		Email: "frotz@example.com",
		Login: "frotz",
	})
	repo2 := result.Repository
	parent2 := mutator.UpsertTopic(in.UpsertTopicOptions{Name: "Parent 2", Repository: repo2})

	mutator.UpsertLink(in.UpsertLinkOptions{
		Title:          "Repo 1 link",
		URL:            "https://example.com/some-url",
		Repository:     repo1,
		ParentTopicIds: []string{parent1.ID, parent2.ID},
	})

	view := &models.View{ViewerID: user1.ID}

	query := queries.LinkQuery{View: view, Topic: parent1.Topic, Viewer: user1}
	links, err := query.Fetch(context.Background(), in.DB)
	in.Must(err)

	for _, link := range links {
		if len(link.R.ParentTopics) < 1 {
			t.Fatalf("Expected at least one eager-loaded parent topic for %s", link)
		}

		for _, parentTopic := range link.R.ParentTopics {
			if parentTopic.ID == parent2.ID {
				t.Fatalf("Topic %s should not be visible to %s", parent2, user1)
			}
		}
	}
}
