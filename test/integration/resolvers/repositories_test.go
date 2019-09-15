package resolvers_test

import (
	"fmt"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
)

func TestRootTopic(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()

	defaultRepo, err := testViewer.DefaultRepo(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	rootTopic, err := defaultRepo.RootTopic(m.ctx, testDB, m.actor.DefaultView())
	if err != nil {
		t.Fatal(err)
	}

	path := fmt.Sprintf("/%s/topics/%s", testViewer.Login.String, rootTopic.ID)
	repoResolver := rootResolver.Repository()

	topic, err := repoResolver.RootTopic(ctx, defaultRepo)
	if err != nil {
		t.Fatal(err)
	}

	topicResolver := rootResolver.Topic()
	var rootPath string

	if rootPath, err = topicResolver.ResourcePath(m.ctx, topic); err != nil {
		t.Fatal(err)
	}

	if path != rootPath {
		t.Fatalf("Unexpected root path: %s", rootPath)
	}
}

func TestSelectRepository(t *testing.T) {
	ctx := testContext()
	m := newMutator(t, testViewer)

	repo, err := testViewer.SelectedRepository().One(ctx, testDB)
	if err == nil && repo != nil {
		if err = testViewer.RemoveSelectedRepository(ctx, m.db, repo); err != nil {
			t.Fatal(err)
		}
	}

	if err = testViewer.Reload(ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if testViewer.SelectedRepositoryID.Ptr() != nil {
		t.Fatalf("Expected user to not have a repository selected: %v", testViewer.SelectedRepositoryID)
	}

	repo = m.defaultRepo()

	payload, err := m.resolver.SelectRepository(ctx, models.SelectRepositoryInput{RepositoryID: &repo.ID})
	if err != nil {
		t.Fatal(err)
	}

	if payload.Repository.ID != repo.ID {
		t.Fatal("Expected the repository to be the one that was selected")
	}
}
