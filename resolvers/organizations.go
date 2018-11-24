package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type organizationResolver struct {
	*Resolver
}

// CreatedAt returns the time of the organization's creation.
func (r *organizationResolver) CreatedAt(
	_ context.Context, org *models.Organization,
) (string, error) {
	return org.CreatedAt.Format(time.RFC3339), nil
}

// Link returns a specific link.
// Links returns a set of links.
func (r *organizationResolver) Link(
	ctx context.Context, org *models.Organization, linkId string,
) (*models.Link, error) {
	return org.Links(qm.Where("id = ?", linkId)).One(ctx, r.DB)
}

// Links returns a set of links.
func (r *organizationResolver) Links(
	ctx context.Context, org *models.Organization, first *int, after *string, last *int,
	before *string,
) (*models.LinkConnection, error) {
	return linkConnection(org.Links(qm.OrderBy("created_at desc")).All(ctx, r.DB))
}

// ResourcePath returns a path to the item.
func (r *organizationResolver) ResourcePath(
	_ context.Context, org *models.Organization,
) (string, error) {
	return "/organizations/" + org.ID, nil
}

// Topic returns a topic for a given id.
func (r *organizationResolver) Topic(
	ctx context.Context, org *models.Organization, id string,
) (*models.Topic, error) {
	return org.Topics(qm.Where("id = ?", id)).One(ctx, r.DB)
}

// Topics returns a set of topics.
func (r *organizationResolver) Topics(
	ctx context.Context, org *models.Organization, first *int, after *string, last *int,
	before *string,
) (*models.TopicConnection, error) {
	return topicConnection(org.Topics(qm.OrderBy("name")).All(ctx, r.DB))
}

// UpdatedAt returns the time the organization was last updated.
func (r *organizationResolver) UpdatedAt(
	_ context.Context, org *models.Organization,
) (string, error) {
	return org.UpdatedAt.Format(time.RFC3339), nil
}
