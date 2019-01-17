package resolvers

//go:generate go run ../scripts/gqlgen.go

import (
	"context"
	"database/sql"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/boil"
)

// CurrentUserKey is the key used for storing the current user in the session.
const CurrentUserKey = "currentUserKey"

var guestUser models.User

func init() {
	guestUser = models.User{}
}

// Resolver is the abstract base class for resolvers.
type Resolver struct {
	DB    *sql.DB
	Actor *models.User
}

// New returns a new resolver.
func New(db *sql.DB, actor *models.User) *Resolver {
	return &Resolver{db, actor}
}

// Mutation returns a resolver that can be used for issuing mutations.
func (r *Resolver) Mutation() models.MutationResolver {
	return &MutationResolver{r}
}

// Query returns a resolver that can be used for issuing queries.
func (r *Resolver) Query() models.QueryResolver {
	return &queryResolver{r}
}

// Link returns an instance of models.LinkResolver.
func (r *Resolver) Link() models.LinkResolver {
	return &linkResolver{r}
}

// Organization returns an instance of models.OrganizationResolver.
func (r *Resolver) Organization() models.OrganizationResolver {
	return &organizationResolver{r}
}

// Repository returns an instance of models.LinkResolver.
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

func getCurrentUser(ctx context.Context, exec boil.ContextExecutor) models.User {
	if false {
		user, err := models.FindUser(ctx, exec, "some-user-id")
		if err != nil {
			panic(err)
		}
		return *user
	}

	value := ctx.Value(CurrentUserKey)
	if user, ok := value.(*models.User); ok {
		return *user
	}
	return guestUser
}

func limitFrom(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}
