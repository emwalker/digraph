package resolvers

//go:generate gorunpkg github.com/99designs/gqlgen

import (
	"context"
	"database/sql"

	"github.com/emwalker/digraph/models"
)

// Resolver is the abstract base class for resolvers.
type Resolver struct {
	DB *sql.DB
}

// Mutation returns a resolver that can be used for issuing mutations.
func (r *Resolver) Mutation() models.MutationResolver {
	return &mutationResolver{r}
}

// Query returns a resolver that can be used for issuing queries.
func (r *Resolver) Query() models.QueryResolver {
	return &queryResolver{r}
}

type mutationResolver struct{ *Resolver }

// CreateTopic creates a new topic.
func (r *mutationResolver) CreateTopic(ctx context.Context, input models.CreateTopicInput) (*models.CreateTopicPayload, error) {
	panic("not implemented")
}

// SelectTopic updates the currently selected topic.
func (r *mutationResolver) SelectTopic(ctx context.Context, input models.SelectTopicInput) (*models.SelectTopicPayload, error) {
	panic("not implemented")
}

// UpsertLink adds a new link to the database.
func (r *mutationResolver) UpsertLink(ctx context.Context, input models.UpsertLinkInput) (*models.UpsertLinkPayload, error) {
	panic("not implemented")
}

type queryResolver struct{ *Resolver }

// Viewer returns the logged-in user.
func (r *queryResolver) Viewer(ctx context.Context) (*models.User, error) {
	return models.Users().One(ctx, r.DB)
}

// Organization returns a resolver that can be used to look up an organization.
func (r *queryResolver) Organization(ctx context.Context, id string) (*models.Organization, error) {
	org, err := models.FindOrganization(ctx, r.DB, id)
	if err != nil {
		return nil, err
	}
	return org, nil
}

// Organization returns an instance of models.OrganizationResolver.
func (r *Resolver) Organization() models.OrganizationResolver {
	return &organizationResolver{r}
}

// Topic returns an instance of models.TopicResolver.
func (r *Resolver) Topic() models.TopicResolver {
	return &topicResolver{r}
}

// User returns an instance of models.UserResolver.
func (r *Resolver) User() models.UserResolver {
	return &userResolver{r}
}

// Link returns an instance of models.LinkResolver.
func (r *Resolver) Link() models.LinkResolver {
	return &linkResolver{r}
}
