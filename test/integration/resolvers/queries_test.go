package resolvers_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/resolvers"
)

func TestResolveView(t *testing.T) {
	ctx := testContext()
	m := newMutator(t, testViewer)

	repo := m.defaultRepo()
	org, err := repo.Organization().One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	queryResolver := rootResolver.Query()

	cases := []struct {
		Name     string
		RepoID   *string
		RepoName *string
		OrgLogin string
	}{
		{
			Name:     "When the org login and repo name are provied",
			RepoName: &repo.Name,
			OrgLogin: org.Login,
		},
		{
			Name:     "When only the org login is provided",
			RepoName: nil,
			OrgLogin: org.Login,
		},
	}

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			view, err := queryResolver.View(
				ctx,
				td.OrgLogin,
				td.RepoName,
				[]string{},
				&testViewer.ID,
			)

			if err != nil {
				t.Fatal(err)
			}

			if repo.ID != view.CurrentRepository.ID {
				t.Fatalf("Expected repo %s, got repo %s", repo.ID, view.CurrentRepository.ID)
			}
		})
	}
}

func TestDefaultOrganization(t *testing.T) {
	ctx := testContext()
	resolver := rootResolver.Query()

	org, err := resolver.DefaultOrganization(ctx)
	if err != nil {
		t.Fatal(err)
	}

	if !org.Public || org.Login != "wiki" {
		t.Fatal("Expected the public organization")
	}
}

func TestFakeError(t *testing.T) {
	queryResolver := rootResolver.Query()

	str, err := queryResolver.FakeError(context.Background())
	if err == nil {
		t.Fatal("Expected a fake error")
	}

	if str != nil {
		t.Fatal("Did not expect a return value")
	}
}

func TestGuestViewer(t *testing.T) {
	ctx := context.Background()
	resolver := resolvers.New(rootResolver.DB, rootResolver.Fetcher, rootResolver.RD).Query()

	viewer, err := resolver.Viewer(ctx)
	if err != nil {
		t.Fatal(err)
	}

	if !viewer.IsGuest() {
		t.Fatal("Expected the guest user")
	}
}
