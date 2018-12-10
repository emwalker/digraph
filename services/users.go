package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
)

type CreateUserResult struct {
	Alerts       []models.Alert
	User         *models.User
	Repository   *models.Repository
	Organization *models.Organization
}

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
		return nil, err
	}

	log.Printf("Making %s an owner of the new default organization", githubUsername)
	member := models.OrganizationMember{
		OrganizationID: org.ID,
		UserID:         user.ID,
		Owner:          true,
	}

	if err = member.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	log.Printf("Creating a default repository for %s", githubUsername)
	repo := models.Repository{
		OrganizationID: org.ID,
		Name:           "system:default",
		OwnerID:        user.ID,
		System:         true,
	}

	if err = repo.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	return &CreateUserResult{
		User:         &user,
		Organization: &org,
		Repository:   &repo,
	}, nil
}
