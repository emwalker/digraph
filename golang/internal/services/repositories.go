package services

import (
	"context"
	"errors"
	"fmt"
	"log"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/volatiletech/sqlboiler/v4/boil"
)

// Error constants.
var (
	ErrInvalidLogin = errors.New("not a valid login")
)

// CreateRepositoryResult holds the result of a CreateRepository service call.
type CreateRepositoryResult struct {
	Repository *models.Repository
	RootTopic  *models.Topic
}

// CreateRepository holds parameters for creating a new repository.
type CreateRepository struct {
	Organization *models.Organization
	Name         string
	Owner        *models.User
	System       bool
}

// Call adds a new repository to the database.
func (m *CreateRepository) Call(ctx context.Context, exec boil.ContextExecutor) (*CreateRepositoryResult, error) {
	login := m.Owner.Login
	name := m.Name

	if !login.Valid {
		return nil, ErrInvalidLogin
	}

	repoName := fmt.Sprintf("%s/%s", login.String, name)

	log.Printf("Creating repository %s", repoName)
	repo := models.Repository{
		OrganizationID: m.Organization.ID,
		Name:           name,
		OwnerID:        m.Owner.ID,
		System:         m.System,
	}

	if err := repo.Insert(ctx, exec, boil.Infer()); err != nil {
		return nil, err
	}

	log.Printf("Creating a root topic for %s", repoName)
	topic := models.Topic{
		OrganizationID: m.Organization.ID,
		RepositoryID:   repo.ID,
		Name:           "Everything",
		Root:           true,
	}

	if err := topic.Insert(ctx, exec, boil.Infer()); err != nil {
		return nil, err
	}

	return &CreateRepositoryResult{
		Repository: &repo,
		RootTopic:  &topic,
	}, nil
}
