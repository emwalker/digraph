package digraph

//go:generate gorunpkg github.com/99designs/gqlgen
//go:generate sqlboiler psql

import (
	"context"

	"github.com/emwalker/digraph/models"
)

// Resolver is the abstract base class for resolvers.
type Resolver struct{}

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
	panic("not implemented")
}

// Organization returns a resolver that can be used to look up an organization.
func (r *queryResolver) Organization(ctx context.Context, resourceID string) (*models.Organization, error) {
	panic("not implemented")
}

type organizationResolver struct{ models.OrganizationResolver }

type topicResolver struct{ models.TopicResolver }

type userResolver struct{ models.TopicResolver }

// Email returns the email of a user.
func (r *userResolver) Email(context.Context, *models.User) (string, error) {
	return "email@email.com", nil
}

// ResourceID returns the resourceID of a user
func (r *userResolver) ResourceID(context.Context, *models.User) (string, error) {
	return "1234", nil
}

// SelectedTopic returns the user's currently selected topic.
func (r *userResolver) SelectedTopic(context.Context, *models.User) (*models.Topic, error) {
	return nil, nil
}

// Organization returns an instance of models.OrganizationResolver.
func (r *Resolver) Organization() models.OrganizationResolver {
	return &organizationResolver{}
}

// Topic returns an instance of models.TopicResolver.
func (r *Resolver) Topic() models.TopicResolver {
	return &topicResolver{}
}

// User returns an instance of models.UserResolver.
func (r *Resolver) User() models.UserResolver {
	return &userResolver{}
}
