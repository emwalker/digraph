package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/services"
	helpers "github.com/emwalker/digraph/testing"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestCreateUser(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, cleanup, err := helpers.CreateUser(
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

	if result.RootTopic == nil {
		t.Fatal("Expected a root topic to be present")
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
