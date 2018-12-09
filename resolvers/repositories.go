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
