package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/pkg/errors"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
)

// CreateUserResult holds the result of a CreateUser call.
type CreateUserResult struct {
	*CreateRepositoryResult
	Alerts       []*models.Alert
	Cleanup      CleanupFunc
	User         *models.User
	Organization *models.Organization
}

func addMember(ctx context.Context, exec boil.ContextExecutor, orgID, userID string, owner bool) error {
	log.Printf("Making user %s a member of organization %s (owner: %t)", orgID, userID, owner)
	member := models.OrganizationMember{
		OrganizationID: orgID,
		UserID:         userID,
		Owner:          owner,
	}

	if err := member.Insert(ctx, exec, boil.Infer()); err != nil {
		return err
	}

	return nil
}

// CreateUser creates a new user and provides a default organization and repo for him/her.
func (c Connection) CreateUser(
	ctx context.Context, name, email, githubUsername, githubAvatarURL string,
) (*CreateUserResult, error) {
	var err error

	log.Printf("Creating user account (%s, %s)", githubUsername, name)
	user := models.User{
		GithubAvatarURL: null.StringFromPtr(&githubAvatarURL),
		GithubUsername:  null.StringFromPtr(&githubUsername),
		Login:           githubUsername,
		Name:            name,
		PrimaryEmail:    email,
	}

	if err = user.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	log.Printf("Creating a default organization for %s", githubUsername)
	org := models.Organization{
		Login:  githubUsername,
		Name:   "system:default",
		Public: false,
		System: true,
	}

	if err = org.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, errors.Wrap(err, "services: failed to insert organization")
	}

	if err = addMember(ctx, c.Exec, org.ID, user.ID, true); err != nil {
		return nil, errors.Wrap(err, "services: failed to add user as a member to new organization")
	}

	if err = addMember(ctx, c.Exec, PublicOrgID, user.ID, false); err != nil {
		return nil, errors.Wrap(err, "services: failed to add user as a member to the public organization")
	}

	log.Printf("Creating a default repo for %s", githubUsername)
	repoResult, err := c.CreateRepository(ctx, &org, "system:default", &user, true)
	if err != nil {
		return nil, errors.Wrap(err, "services: failed to create a default repo for user")
	}

	cleanup := func() error {
		if err := repoResult.Cleanup(); err != nil {
			return err
		}
		if _, err := org.Delete(ctx, c.Exec); err != nil {
			return err
		}
		if _, err := user.Delete(ctx, c.Exec); err != nil {
			return err
		}
		return nil
	}

	return &CreateUserResult{
		Alerts:                 []*models.Alert{},
		CreateRepositoryResult: repoResult,
		Cleanup:                cleanup,
		Organization:           &org,
		User:                   &user,
	}, nil
}
