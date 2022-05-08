package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/golang/cmd/frontend/models"
	"github.com/emwalker/digraph/golang/cmd/frontend/queries"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// CreateSessionResult holds the result of a Create{Github,Google}Session service call.
type CreateSessionResult struct {
	Alerts  []*models.Alert
	Cleanup CleanupFunc
	Session *models.Session
	User    *models.User
}

// CreateGithubSession creates a new session for the user.  If the user is not found in the database,
// a new user is created.
func (c Connection) CreateGithubSession(
	ctx context.Context, name, primaryEmail, githubUsername, githubAvatarURL string,
) (*CreateSessionResult, error) {
	var result *CreateUserResult
	var user *models.User

	account, err := models.GithubAccounts(qm.Where("username = ?", githubUsername)).One(ctx, c.Exec)

	if err != nil {
		if err.Error() != queries.ErrSQLNoRows {
			log.Printf("Unable to upsert user: %s", err)
			return nil, errors.Wrap(err, "services: failed to upsert user")
		}

		createUser := CreateUser{Name: name, Email: primaryEmail, AvatarURL: githubAvatarURL}
		result, err := createUser.Call(ctx, c.Exec)
		if err != nil {
			return nil, err
		}

		user = result.User

		_, err = c.CreateGithubAccount(
			ctx, user, githubUsername, name, primaryEmail, githubAvatarURL,
		)
		if err != nil {
			return nil, err
		}

		completeRegistration := CompleteRegistration{User: user, Login: githubUsername}
		if _, err = completeRegistration.Call(ctx, c.Exec); err != nil {
			return nil, err
		}
	} else {
		user, err = account.User().One(ctx, c.Exec)
		if err != nil {
			return nil, errors.Wrap(err, "services: failed to fetch user")
		}
	}

	session := &models.Session{UserID: user.ID}
	if err = session.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, errors.Wrap(err, "services: failed to create session")
	}

	cleanup := func() error {
		if result != nil {
			if _, err := result.User.Delete(ctx, c.Exec); err != nil {
				return err
			}

			_, err = models.Organizations(qm.Where("login = ?", result.User.Login)).DeleteAll(ctx, c.Exec)
			if err != nil {
				return err
			}
		}

		return nil
	}

	return &CreateSessionResult{
		Cleanup: cleanup,
		Session: session,
		User:    user,
	}, nil
}

// CreateGoogleSession creates a new session for the user.  If the user is not found in the database,
// a new user is created.
func (c Connection) CreateGoogleSession(
	ctx context.Context, name, primaryEmail, profileID, googleAvatarURL string,
) (*CreateSessionResult, error) {
	var result *CreateUserResult
	var user *models.User

	account, err := models.GoogleAccounts(qm.Where("profile_id = ?", profileID)).One(ctx, c.Exec)

	if err != nil {
		if err.Error() != queries.ErrSQLNoRows {
			log.Printf("Unable to upsert user: %s", err)
			return nil, errors.Wrap(err, "services: failed to upsert user")
		}

		service := CreateUser{Name: name, Email: primaryEmail, AvatarURL: googleAvatarURL}
		result, err := service.Call(ctx, c.Exec)
		if err != nil {
			return nil, err
		}

		user = result.User

		_, err = c.CreateGoogleAccount(
			ctx, user, profileID, name, primaryEmail, googleAvatarURL,
		)
		if err != nil {
			return nil, err
		}

	} else {
		user, err = account.User().One(ctx, c.Exec)
		if err != nil {
			return nil, errors.Wrap(err, "services: failed to fetch user")
		}
	}

	session := &models.Session{UserID: user.ID}
	if err = session.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, errors.Wrap(err, "services: failed to create session")
	}

	cleanup := func() error {
		if result != nil {
			if _, err := result.User.Delete(ctx, c.Exec); err != nil {
				return err
			}

			_, err = models.Organizations(qm.Where("login = ?", result.User.Login)).DeleteAll(ctx, c.Exec)
			if err != nil {
				return err
			}
		}

		return nil
	}

	return &CreateSessionResult{
		Cleanup: cleanup,
		Session: session,
		User:    user,
	}, nil
}
