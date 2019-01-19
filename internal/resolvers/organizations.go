package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/internal/loaders"
	"github.com/emwalker/digraph/internal/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type organizationResolver struct {
	*Resolver
}

func getOrganizationLoader(ctx context.Context) *loaders.OrganizationLoader {
	return ctx.Value(loaders.OrganizationLoaderKey).(*loaders.OrganizationLoader)
}

func fetchOrganization(ctx context.Context, organizationID string) (models.Organization, error) {
	loader := getOrganizationLoader(ctx)
	org, err := loader.Load(organizationID)
	if err != nil {
		return models.Organization{}, err
	}
	return *org, nil
}

// CreatedAt returns the time of the organization's creation.
func (r *organizationResolver) CreatedAt(_ context.Context, org *models.Organization) (string, error) {
	return org.CreatedAt.Format(time.RFC3339), nil
}

// DefaultRepository returns the default repository for the organization.
func (r *organizationResolver) DefaultRepository(ctx context.Context, org *models.Organization) (models.Repository, error) {
	repo, err := org.Repositories(qm.Where("system")).One(ctx, r.DB)
	if err != nil {
		return models.Repository{}, err
	}
	return *repo, nil
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
