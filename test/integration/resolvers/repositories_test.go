package resolvers_test

import (
	"context"
	"fmt"
	"testing"

	"github.com/emwalker/digraph/internal/models"
	"github.com/emwalker/digraph/internal/resolvers"
)

func TestRootTopic(t *testing.T) {
	m := newMutator(t, testActor)
	resolver := &resolvers.Resolver{DB: testDB}

	defaultRepo, err := testActor.DefaultRepo(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	rootTopic, err := defaultRepo.RootTopic(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	path := fmt.Sprintf("/%s/topics/%s", testActor.Login, rootTopic.ID)
	repoResolver := resolver.Repository()

	topic, err := repoResolver.RootTopic(context.Background(), defaultRepo)
	if err != nil {
		t.Fatal(err)
	}

	topicResolver := resolver.Topic()
	var rootPath string

	if rootPath, err = topicResolver.ResourcePath(m.ctx, &topic); err != nil {
		t.Fatal(err)
	}

	if path != rootPath {
		t.Fatalf("Unexpected root path: %s", rootPath)
	}
}

func TestSelectRepository(t *testing.T) {
	m := newMutator(t, testActor)
	ctx := context.Background()

	repo, err := testActor.SelectedRepository().One(ctx, testDB)
	if err == nil && repo != nil {
		if err = testActor.RemoveSelectedRepository(ctx, m.db, repo); err != nil {
			t.Fatal(err)
		}
	}

	if err = testActor.Reload(ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if testActor.SelectedRepositoryID.Ptr() != nil {
		t.Fatalf("Expected user to not have a repository selected: %v", testActor.SelectedRepositoryID)
	}

	repo = m.defaultRepo()

	ctx = context.WithValue(ctx, resolvers.CurrentUserKey, testActor)
	payload, err := m.resolver.SelectRepository(ctx, models.SelectRepositoryInput{RepositoryID: &repo.ID})
	if err != nil {
		t.Fatal(err)
	}

	if payload.Repository.ID != repo.ID {
		t.Fatal("Expected the repository to be the one that was selected")
	}
}
