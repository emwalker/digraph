package queries_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries"
	in "github.com/emwalker/digraph/test/integration"
)

func TestTopicParentTopicsAreFiltered(t *testing.T) {
	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteRepositoriesByName("Repo 1", "Repo 2")
	mutator.DeleteTopicsByName("Repo 1 topic", "Parent 1", "Parent 2")
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

	topic := mutator.UpsertTopic(in.UpsertTopicOptions{
		Name:           "Repo 1 topic",
		Repository:     repo1,
		ParentTopicIds: []string{parent1.ID, parent2.ID},
	})

	view := &models.View{ViewerID: user1.ID}

	query := queries.TopicParentTopics{View: view, Topic: topic.Topic}
	parentTopics, err := query.Fetch(context.Background(), in.DB)
	in.Must(err)

	for _, topic := range parentTopics {
		if topic.ID == parent2.ID {
			t.Fatalf("Expected %s to be hidden from %s, but it was shown", parent2, user1)
		}
	}
}

func TestLinkParentTopicsAreFiltered(t *testing.T) {
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

	link := mutator.UpsertLink(in.UpsertLinkOptions{
		Title:          "Repo 1 link",
		URL:            "https://example.com/some-url",
		Repository:     repo1,
		ParentTopicIds: []string{parent1.ID, parent2.ID},
	})

	view := &models.View{ViewerID: user1.ID}

	query := queries.LinkParentTopics{View: view, Link: link}
	parentTopics, err := query.Fetch(context.Background(), in.DB)
	in.Must(err)

	for _, topic := range parentTopics {
		if topic.ID == parent2.ID {
			t.Fatalf("Expected %s to be hidden from %s, but it was shown", parent2, user1)
		}
	}
}
