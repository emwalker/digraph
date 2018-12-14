package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

type repositoryResolver struct {
	*Resolver
}

// Organization returns a set of links.
func (r *repositoryResolver) Organization(
	ctx context.Context, repo *models.Repository,
) (models.Organization, error) {
	org, err := repo.Organization().One(ctx, r.DB)
	return *org, err
}

// Organization returns a set of links.
func (r *repositoryResolver) Owner(
	ctx context.Context, repo *models.Repository,
) (models.User, error) {
	org, err := repo.Owner().One(ctx, r.DB)
	return *org, err
}

// RootTopic returns the root topic of the repository.
func (r *repositoryResolver) RootTopic(
	ctx context.Context, repo *models.Repository,
) (models.Topic, error) {
	topic, err := repo.RootTopic(ctx, r.DB)
	if err != nil {
		return models.Topic{}, err
	}
	return *topic, err
}
