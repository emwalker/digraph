package services

import (
	"context"
	"log"

	"github.com/emwalker/digraph/internal/models"
	"github.com/volatiletech/sqlboiler/boil"
)

// CreateRepositoryResult holds the result of a CreateRepository service call.
type CreateRepositoryResult struct {
	Cleanup    CleanupFunc
	Repository *models.Repository
	RootTopic  *models.Topic
}

// CreateRepository adds a new repository to the database.
func (c *Connection) CreateRepository(
	ctx context.Context, org *models.Organization, name string, owner *models.User, system bool,
) (*CreateRepositoryResult, error) {
	log.Printf("Creating repository %s", name)
	repo := models.Repository{
		OrganizationID: org.ID,
		Name:           name,
		OwnerID:        owner.ID,
		System:         system,
	}

	if err := repo.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	log.Printf("Creating a root topic for %s", name)
	topic := models.Topic{
		OrganizationID: org.ID,
		RepositoryID:   repo.ID,
		Name:           "Everything",
		Root:           true,
	}

	if err := topic.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	cleanup := func() error {
		if _, err := repo.Delete(ctx, c.Exec); err != nil {
			return err
		}
		if _, err := topic.Delete(ctx, c.Exec); err != nil {
			return err
		}
		return nil
	}

	return &CreateRepositoryResult{cleanup, &repo, &topic}, nil
}
