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

type FetchOrMakeSessionSessionResult struct {
	Session        *models.Session
	SessionCreated bool
	User           *models.User
	UserCreated    bool
}

func FetchOrMakeSession(
	ctx context.Context, exec boil.ContextExecutor, gothUser goth.User,
) (*FetchOrMakeSessionSessionResult, error) {
	if gothUser.Provider != "github" {
		return nil, errors.New("Do not know how to look up a non-Github user yet")
	}

	session, err := models.Sessions(
		qm.Select("sessions.id id", "sessions.user_id user_id", "encode(sessions.session_id::bytea, 'hex') session_id"),
		qm.InnerJoin("users u on sessions.user_id = u.id"),
		qm.Where("github_username like ?", gothUser.NickName),
	).One(ctx, exec)

	if err != nil && err.Error() != rowNotFound {
		return nil, err
	}

	if session != nil {
		user, err := session.User().One(ctx, exec)
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

	user, err := models.Users(mods...).One(ctx, exec)
	if err != nil && err.Error() != rowNotFound {
		return nil, err
	}

	if user == nil {
		log.Printf("User with a github username '%s' not found, creating", gothUser.NickName)
		user = &models.User{
			Name:            gothUser.Name,
			PrimaryEmail:    gothUser.Email,
			GithubUsername:  null.StringFrom(gothUser.NickName),
			GithubAvatarURL: null.StringFrom(gothUser.AvatarURL),
		}
		if err = user.Insert(ctx, exec, boil.Infer()); err != nil {
			return nil, err
		}
		userCreated = true
	} else {
		log.Printf("Updating github account info for user %s", user.ID)
		user.GithubUsername = null.StringFrom(gothUser.NickName)
		user.GithubAvatarURL = null.StringFrom(gothUser.AvatarURL)
		if _, err = user.Update(ctx, exec, boil.Infer()); err != nil {
			return nil, err
		}
	}

	session = &models.Session{UserID: user.ID}
	if err = session.Insert(ctx, exec, boil.Infer()); err != nil {
		return nil, err
	}

	return &FetchOrMakeSessionSessionResult{
		Session:        session,
		SessionCreated: true,
		User:           user,
		UserCreated:    userCreated,
	}, nil
}
