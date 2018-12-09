package resolvers

//go:generate go run ../scripts/gqlgen.go

import (
	"context"
	"database/sql"

	"github.com/emwalker/digraph/models"
)

// Resolver is the abstract base class for resolvers.
type Resolver struct {
	DB    *sql.DB
	Actor *models.User
}

// Mutation returns a resolver that can be used for issuing mutations.
func (r *Resolver) Mutation() models.MutationResolver {
	return &MutationResolver{r}
}

// Query returns a resolver that can be used for issuing queries.
func (r *Resolver) Query() models.QueryResolver {
	return &queryResolver{r}
}

type MutationResolver struct {
	*Resolver
}

type queryResolver struct{ *Resolver }

func getCurrentUser(ctx context.Context) *models.User {
	value := ctx.Value("currentUser")
	if user, ok := value.(*models.User); ok {
		return user
	}
	return nil
}

// Link returns an instance of models.LinkResolver.
func (r *Resolver) Link() models.LinkResolver {
	return &linkResolver{r}
}

// Organization returns an instance of models.OrganizationResolver.
func (r *Resolver) Organization() models.OrganizationResolver {
	return &organizationResolver{r}
}

// Link returns an instance of models.LinkResolver.
func (r *Resolver) Repository() models.RepositoryResolver {
	return &repositoryResolver{r}
}

// Topic returns an instance of models.TopicResolver.
func (r *Resolver) Topic() models.TopicResolver {
	return &topicResolver{r}
}

// User returns an instance of models.UserResolver.
func (r *Resolver) User() models.UserResolver {
	return &userResolver{r}
}

// View returns an instance of models.ViewResolver
func (r *Resolver) View() models.ViewResolver {
	return &viewResolver{r}
}

// Viewer returns the logged-in user.
func (r *queryResolver) Viewer(ctx context.Context) (*models.User, error) {
	return getCurrentUser(ctx), nil
}

// View returns a resolver that filters results on the basis of one or more organizations.
func (r *queryResolver) View(ctx context.Context, organizationIds []string) (models.View, error) {
	return models.View{OrganizationIds: organizationIds}, nil
}
