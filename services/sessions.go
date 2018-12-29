package services

import (
	"context"
	"errors"
	"log"

	"github.com/emwalker/digraph/models"
	"github.com/markbates/goth"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

const rowNotFound = "sql: no rows in result set"

// FetchOrMakeSessionSessionResult holds the result of a FetchOrMakeSessionSession service call.
type FetchOrMakeSessionSessionResult struct {
	Session        *models.Session
	SessionCreated bool
	User           *models.User
	UserCreated    bool
}

// FetchOrMakeSession obtains the current session if it exists.
func (c Connection) FetchOrMakeSession(
	ctx context.Context, gothUser goth.User,
) (*FetchOrMakeSessionSessionResult, error) {
	if gothUser.Provider != "github" {
		return nil, errors.New("Do not know how to look up a non-Github user yet")
	}

	session, err := models.Sessions(
		qm.Select(
			"sessions.id id",
			"sessions.user_id user_id",
			"encode(sessions.session_id::bytea, 'hex') session_id",
		),
		qm.InnerJoin("users u on sessions.user_id = u.id"),
		qm.Where("github_username like ?", gothUser.NickName),
	).One(ctx, c.Exec)

	if err != nil && err.Error() != rowNotFound {
		return nil, err
	}

	if session != nil {
		user, err := session.User().One(ctx, c.Exec)
		if err != nil {
			return nil, err
		}

		return &FetchOrMakeSessionSessionResult{
			Session:        session,
			SessionCreated: false,
			User:           user,
			UserCreated:    false,
		}, nil
	}

	userCreated := false

	mods := []qm.QueryMod{
		qm.Where("github_username like ?", gothUser.NickName),
	}

	if gothUser.Email != "" {
		mods = append(mods, qm.Or("primary_email like ?", gothUser.Email))
	}

	user, err := models.Users(mods...).One(ctx, c.Exec)
	if err != nil && err.Error() != rowNotFound {
		return nil, err
	}

	if user == nil {
		log.Printf("User with a github username '%s' not found, creating", gothUser.NickName)
		result, err := c.CreateUser(
			ctx,
			gothUser.Name,
			gothUser.Email,
			gothUser.NickName,
			gothUser.AvatarURL,
		)

		if err != nil {
			return nil, err
		}

		user = result.User
		userCreated = true
	} else {
		log.Printf("Updating github account info for user %s", user.ID)
		user.GithubUsername = null.StringFrom(gothUser.NickName)
		user.GithubAvatarURL = null.StringFrom(gothUser.AvatarURL)
		if _, err = user.Update(ctx, c.Exec, boil.Infer()); err != nil {
			return nil, err
		}
	}

	session = &models.Session{UserID: user.ID}
	if err = session.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	return &FetchOrMakeSessionSessionResult{
		Session:        session,
		SessionCreated: true,
		User:           user,
		UserCreated:    userCreated,
	}, nil
}
