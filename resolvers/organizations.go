package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type organizationResolver struct {
	*Resolver
}

// ResourcePath returns a path to the item.
func (r *organizationResolver) ResourcePath(_ context.Context, org *models.Organization) (string, error) {
	return "/organizations/" + org.ID, nil
}

// Topic returns a topic for a given id.
func (r *organizationResolver) Topic(ctx context.Context, org *models.Organization, id string) (*models.Topic, error) {
	return org.Topics(qm.Where("id = ?", id)).One(ctx, r.DB)
}

// Topics returns a set of topics.
func (r *organizationResolver) Topics(ctx context.Context, org *models.Organization, first *int, after *string, last *int, before *string) (*models.TopicConnection, error) {
	return topicConnection(org.Topics().All(ctx, r.DB))
}

// Links returns a set of links.
func (r *organizationResolver) Links(ctx context.Context, org *models.Organization, first *int, after *string, last *int, before *string) (*models.LinkConnection, error) {
	return linkConnection(org.Links().All(ctx, r.DB))
}
