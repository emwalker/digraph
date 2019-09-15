package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestCreateUser(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.CreateUser(ctx, "Gnusto Frotz", "gnusto@gnusto.com", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if result.User == nil {
		t.Fatal("Expected a user to be present")
	}

	count, err := models.OrganizationMembers(
		qm.Where("user_id = ?", result.User.ID),
		qm.Where("organization_id = ?", services.PublicOrgID),
	).Count(ctx, testDB)

	if count < 1 {
		t.Fatal("Expected new user to be added to the public org")
	}
}

func TestCompleteRegistration(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	userResult, err := c.CreateUser(ctx, "Gnusto Frotz", "gnusto@gnusto.com", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer userResult.Cleanup()

	user := userResult.User

	result, err := c.CompleteRegistration(ctx, user, "gnusto")
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if result.Organization == nil {
		t.Fatal("Expected an organization to be present")
	}

	if result.Repository == nil {
		t.Fatal("Expected a repo to be present")
	}

	if result.RootTopic == nil {
		t.Fatal("Expected a root topic to be present")
	}

	membership, err := models.OrganizationMembers(
		qm.Where("organization_id = ? and user_id = ?", result.Organization.ID, user.ID),
	).One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if !membership.Owner {
		t.Fatal("Expected the user to be made owner of the new organization")
	}

	// It adds the new user as a member to the public org
	_, err = models.OrganizationMembers(
		qm.Where("organization_id = ? and user_id = ?", services.PublicOrgID, user.ID),
	).One(ctx, testDB)
	if err != nil {
		t.Fatalf("User was not added to the public org: %s", err)
	}
}
