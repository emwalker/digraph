package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/loaders"
	"github.com/emwalker/digraph/models"
)

type organizationResolver struct {
	*Resolver
}

func getOrganizationLoader(ctx context.Context) *loaders.OrganizationLoader {
	return ctx.Value(loaders.OrganizationLoaderKey).(*loaders.OrganizationLoader)
}

func fetchOrganization(ctx context.Context, organizationId string) (models.Organization, error) {
	loader := getOrganizationLoader(ctx)
	org, err := loader.Load(organizationId)
	if err != nil {
		return models.Organization{}, err
	}
	return *org, nil
}

// CreatedAt returns the time of the organization's creation.
func (r *organizationResolver) CreatedAt(
	_ context.Context, org *models.Organization,
) (string, error) {
	return org.CreatedAt.Format(time.RFC3339), nil
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
