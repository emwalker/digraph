package resolvers

//go:generate go run ../../../scripts/gqlgen.go

import (
	"context"
	"database/sql"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/go-redis/redis"
)

// CurrentUserKey is the key used for storing the current user in the session.
const (
	CurrentUserKey      = "currentUserKey"
	EverythingTopicPath = "/wiki/topics/df63295e-ee02-11e8-9e36-17d56b662bc8"
	generalRepositoryID = "32212616-fc1b-11e8-8eda-b70af6d8d09f"
)

// GuestUser is a placeholder user that is used when someone visits the app without a session.
var GuestUser models.User

func init() {
	GuestUser = models.User{}
}

// Resolver is the abstract base class for resolvers.
type Resolver struct {
	DB      *sql.DB
	Actor   *models.User
	Fetcher pageinfo.Fetcher
	RD      *redis.Client
}

// New returns a new resolver.
func New(db *sql.DB, actor *models.User, fetcher pageinfo.Fetcher, rd *redis.Client) *Resolver {
	return &Resolver{
		DB:      db,
		Actor:   actor,
		Fetcher: fetcher,
		RD:      rd,
	}
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

// Synonym returns an instance of models.UserResolver.
func (r *Resolver) Synonym() models.SynonymResolver {
	return &synonymResolver{r}
}

// User returns an instance of models.UserResolver.
func (r *Resolver) User() models.UserResolver {
	return &userResolver{r}
}

// View returns an instance of models.ViewResolver
func (r *Resolver) View() models.ViewResolver {
	return &viewResolver{r}
}

func getCurrentUser(ctx context.Context) models.User {
	value := ctx.Value(CurrentUserKey)
	if user, ok := value.(*models.User); ok {
		return *user
	}
	return GuestUser
}

func limitFrom(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}
