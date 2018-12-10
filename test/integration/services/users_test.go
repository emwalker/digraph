package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func createUser(
	c services.Connection, ctx context.Context, name, email, githubUsername, githubAvatarURL string,
) (*services.CreateUserResult, cleanupFunc, error) {
	result, err := c.CreateUser(ctx, name, email, githubUsername, githubAvatarURL)

	if err != nil {
		return nil, nil, err
	}

	cleanup := func() error {
		result.User.Delete(ctx, c.Exec)
		result.Organization.Delete(ctx, c.Exec)
		return nil
	}

	return result, cleanup, nil
}

func TestCreateUser(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, cleanup, err := createUser(
		c,
		ctx,
		"Gnusto Frotz",
		"gnusto@gnusto.com",
		"gnusto",
		"http://some-long-url",
	)
	defer cleanup()

	if err != nil {
		t.Fatal(err)
	}

	if result.User == nil {
		t.Fatal("Expected a user to be present")
	}

	if result.Organization == nil {
		t.Fatal("Expected an organization to be present")
	}

	if result.Repository == nil {
		t.Fatal("Expected a repo to be present")
	}

	membership, err := models.OrganizationMembers(
		qm.Where("organization_id = ? and user_id = ?", result.Organization.ID, result.User.ID),
	).One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if !membership.Owner {
		t.Fatal("Expected the user to be made owner of the new organization")
	}
}
