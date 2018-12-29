package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
)

// CreateUserResult holds the result of a CreateUser call.
type CreateUserResult struct {
	Alerts       []models.Alert
	Cleanup      CleanupFunc
	User         *models.User
	Repository   *models.Repository
	Organization *models.Organization
	RootTopic    *models.Topic
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

	log.Printf("Creating a root topic for %s", githubUsername)
	topic := models.Topic{
		OrganizationID: org.ID,
		RepositoryID:   repo.ID,
		Name:           "Everything",
		Root:           true,
	}

	if err = topic.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	cleanup := func() error {
		if _, err := repo.Delete(ctx, c.Exec); err != nil {
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
		Cleanup:      cleanup,
		Organization: &org,
		Repository:   &repo,
		RootTopic:    &topic,
		User:         &user,
	}, nil
}
