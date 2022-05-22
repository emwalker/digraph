package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/golang/internal/loaders"
	"github.com/emwalker/digraph/golang/internal/models"
)

type organizationResolver struct {
	*Resolver
}

// CreatedAt returns the time of the organization's creation.
func (r *organizationResolver) CreatedAt(_ context.Context, org *models.Organization) (string, error) {
	return org.CreatedAt.Format(time.RFC3339), nil
}

// DefaultRepository returns the default repository for the organization.
func (r *organizationResolver) DefaultRepository(ctx context.Context, org *models.Organization) (*models.Repository, error) {
	return loaders.GetDefaultRepoByOrgID(ctx, org.ID)
}

// ResourcePath returns a path to the item.
func (r *organizationResolver) ResourcePath(
	_ context.Context, org *models.Organization,
) (string, error) {
	return "/organizations/" + org.ID, nil
}

// UpdatedAt returns the time the organization was last updated.
func (r *organizationResolver) UpdatedAt(
	_ context.Context, org *models.Organization,
) (string, error) {
	return org.UpdatedAt.Format(time.RFC3339), nil
}
