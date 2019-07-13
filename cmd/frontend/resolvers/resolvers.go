package resolvers

//go:generate go run ../../../scripts/gqlgen.go

import (
	"database/sql"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/go-redis/redis"
)

// CurrentUserKey is the key used for storing the current user in the session.
const (
	EverythingTopicPath = "/wiki/topics/df63295e-ee02-11e8-9e36-17d56b662bc8"
	TestSessionID       = "f1aaed32-7548-4f7c-920e-d4dc9172e475"
	generalRepositoryID = "32212616-fc1b-11e8-8eda-b70af6d8d09f"
)

// Resolver is the abstract base class for resolvers.
type Resolver struct {
	DB      *sql.DB
	Fetcher pageinfo.Fetcher
	RD      *redis.Client
}

// New returns a new resolver.
func New(db *sql.DB, fetcher pageinfo.Fetcher, rd *redis.Client) *Resolver {
	return &Resolver{
		DB:      db,
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

// Synonym returns an instance of models.SynonymResolver.
func (r *Resolver) Synonym() models.SynonymResolver {
	return &synonymResolver{r}
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

func limitFrom(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}
