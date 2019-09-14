package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/pkg/errors"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// CreateUserResult holds the result of a CreateUser call.
type CreateUserResult struct {
	*CreateRepositoryResult
	Alerts       []*models.Alert
	Cleanup      CleanupFunc
	User         *models.User
	Organization *models.Organization
}

// DeleteAccountResult holds the result of deleting an account.
type DeleteAccountResult struct {
	Alerts        []*models.Alert
	DeletedUserID string
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

// DeleteAccount hard-deletes the account and any private data associated with it.  Topics and links
// added to public repos are not deleted.
func (c Connection) DeleteAccount(
	ctx context.Context, user *models.User,
) (*DeleteAccountResult, error) {
	log.Printf("Deleting account %s ...", user)

	deletedUser := models.DeletedUser{UserID: user.ID}
	if err := deletedUser.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		log.Printf("Failed to add a row to the deleted users table for %s", user.ID)
		return nil, errors.Wrap(err, "services: failed to add a row to deleted users")
	}

	_, err := models.Organizations(qm.Where("login like ? and system", user.Login)).DeleteAll(ctx, c.Exec)
	if err != nil {
		log.Printf("There was a problem deleting organizations for %s: %s", user.ID, err)
		return nil, errors.Wrap(err, "services: failed to fetch organization")
	}
	log.Printf("Organization for %s deleted.", user)

	if _, err := user.Delete(ctx, c.Exec); err != nil {
		return nil, errors.Wrap(err, "services: failed to delete user")
	}
	log.Printf("Account for %s deleted.", user)

	return &DeleteAccountResult{
		Alerts: []*models.Alert{
			models.NewAlert(
				models.AlertTypeSuccess,
				"Your account has been deleted",
			),
		},
		DeletedUserID: user.ID,
	}, nil
}
