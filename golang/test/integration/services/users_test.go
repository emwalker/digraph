package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/services"
	in "github.com/emwalker/digraph/golang/test/integration"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

func TestCreateUser(t *testing.T) {
	ctx := context.Background()

	service := services.CreateUser{Name: "Gnusto Frotz", Email: "gnusto@gnusto.com", AvatarURL: "http://avatar/url"}
	result, err := service.Call(ctx, testDB)
	in.Must(err)

	if result.User == nil {
		t.Fatal("Expected a user to be present")
	}

	count, _ := models.OrganizationMembers(
		qm.Where("user_id = ?", result.User.ID),
		qm.Where("organization_id = ?", services.PublicOrgID),
	).Count(ctx, testDB)

	if count < 1 {
		t.Fatal("Expected new user to be added to the public org")
	}
}

func TestCompleteRegistration(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})

	mutator.DeleteUsersByEmail("gnusto@example.com")
	mutator.DeleteRepositoriesByName("gnusto")
	mutator.DeleteOrganizationsByLogin("gnusto")

	userService := services.CreateUser{Name: "Gnusto Frotz", Email: "gnusto@example.com", AvatarURL: "http://avatar/url"}
	userResult, err := userService.Call(ctx, in.DB)
	in.Must(err)

	user := userResult.User

	completeRegistrationService := services.CompleteRegistration{User: user, Login: "gnusto"}
	result, err := completeRegistrationService.Call(ctx, in.DB)
	in.Must(err)

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
	in.Must(err)

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
