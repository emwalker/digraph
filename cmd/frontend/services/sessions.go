package services

import (
	"context"
	"fmt"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

const rowNotFound = "sql: no rows in result set"

// CreateSessionResult holds the result of a CreateSession service call.
type CreateSessionResult struct {
	Alerts  []*models.Alert
	Cleanup CleanupFunc
	Session *models.Session
	User    *models.User
}

// CreateSession creates a new session for the user.  If the user is not found in the database,
// a new user is created.
func (c Connection) CreateSession(
	ctx context.Context, username, primaryEmail, githubUsername, githubAvatarURL string,
) (*CreateSessionResult, error) {
	var result *CreateUserResult

	user, err := models.Users(qm.Where("primary_email = ?", primaryEmail)).One(ctx, c.Exec)

	if err != nil {
		if err.Error() != "sql: no rows in result set" {
			log.Printf("Unable to upsert user: %s", err)
			return nil, errors.Wrap(err, "resolvers: failed to upsert user")
		}

		result, err := c.CreateUser(
			ctx, username, primaryEmail, githubUsername, githubAvatarURL,
		)

		if err != nil {
			return nil, fmt.Errorf("Unable to create user: %s", err)
		}

		user = result.User
		if err = user.Reload(ctx, c.Exec); err != nil {
			return nil, fmt.Errorf("Unable to reload newly-created user: %s", err)
		}
	}

	session := &models.Session{UserID: user.ID}
	if err = session.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, errors.Wrap(err, "resolvers: failed to create session")
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
