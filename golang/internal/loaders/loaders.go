package loaders

import (
	"context"
	"log"
	"time"

	"github.com/emwalker/digraph/golang/internal"
	"github.com/emwalker/digraph/golang/internal/models"
)

type fetcherConfig struct {
	repo internal.Repo
}

type loaderKeyType string

const (
	loaderKey       = loaderKeyType("loaders")
	defaultMaxBatch = 100
	deafultWaitTime = 2 * time.Millisecond
)

type loaders struct {
	OrganizationByID         *OrganizationLoader
	RepositoryByID           *RepositoryLoader
	DefaultRepositoryByOrgID *RepositoryLoader
	TopicByID                *TopicLoader
}

type fetcher[T identifable] func(ids []string) ([]T, []error)

type identifable interface {
	GetID() string
}

func orderByID[T identifable](ids []string, array []T) []T {
	itemMap := make(map[string]T, len(array))
	for _, item := range array {
		itemMap[item.GetID()] = item
	}
	result := make([]T, len(array))
	for i, id := range ids {
		result[i] = itemMap[id]
	}
	return result
}

func rowsOrError[T identifable](ids []string, array []T, err error) ([]T, []error) {
	if err != nil {
		log.Printf("loaders: error fetching ids %s: %s", ids, err)
		return nil, []error{err}
	}
	return orderByID(ids, array), nil
}

func (f fetcherConfig) topicsByID(ctx context.Context) fetcher[*models.Topic] {
	return func(ids []string) ([]*models.Topic, []error) {
		rows, err := f.repo.GetTopicsByID(ctx, ids)
		return rowsOrError(ids, rows, err)
	}
}

func (f fetcherConfig) orgsByID(ctx context.Context) fetcher[*models.Organization] {
	return func(ids []string) ([]*models.Organization, []error) {
		rows, err := f.repo.GetOrgsByID(ctx, ids)
		return rowsOrError(ids, rows, err)
	}
}

func (f fetcherConfig) reposByID(ctx context.Context) fetcher[*models.Repository] {
	return func(ids []string) ([]*models.Repository, []error) {
		rows, err := f.repo.GetReposByID(ctx, ids)
		return rowsOrError(ids, rows, err)
	}
}

func (f fetcherConfig) defaultRepoByOrgID(ctx context.Context) fetcher[*models.Repository] {
	return func(ids []string) ([]*models.Repository, []error) {
		rows, err := f.repo.GetDefaultReposByOrgID(ctx, ids)
		if err != nil {
			return nil, []error{err}
		}
		itemMap := make(map[string]*models.Repository, len(rows))
		for _, repo := range rows {
			itemMap[repo.OrganizationID] = repo
		}
		result := make([]*models.Repository, len(rows))
		for i, id := range ids {
			result[i] = itemMap[id]
		}
		return result, nil
	}
}

func AddToContext(ctx context.Context, repo internal.Repo) context.Context {
	c := fetcherConfig{repo}

	loaders := loaders{
		TopicByID: NewTopicLoader(TopicLoaderConfig{
			MaxBatch: defaultMaxBatch,
			Wait:     deafultWaitTime,
			Fetch:    c.topicsByID(ctx),
		}),
		OrganizationByID: NewOrganizationLoader(OrganizationLoaderConfig{
			MaxBatch: defaultMaxBatch,
			Wait:     deafultWaitTime,
			Fetch:    c.orgsByID(ctx),
		}),
		RepositoryByID: NewRepositoryLoader(RepositoryLoaderConfig{
			Wait:     deafultWaitTime,
			MaxBatch: defaultMaxBatch,
			Fetch:    c.reposByID(ctx),
		}),
		DefaultRepositoryByOrgID: NewRepositoryLoader(RepositoryLoaderConfig{
			Wait:     deafultWaitTime,
			MaxBatch: defaultMaxBatch,
			Fetch:    c.defaultRepoByOrgID(ctx),
		}),
	}

	ctx = context.WithValue(ctx, loaderKey, &loaders)
	return ctx
}

func For(ctx context.Context) *loaders {
	return ctx.Value(loaderKey).(*loaders)
}

func GetRepo(ctx context.Context, repoID string) (*models.Repository, error) {
	return For(ctx).RepositoryByID.Load(repoID)
}

func GetDefaultRepoByOrgID(ctx context.Context, orgID string) (*models.Repository, error) {
	return For(ctx).DefaultRepositoryByOrgID.Load(orgID)
}

func GetOrg(ctx context.Context, orgID string) (*models.Organization, error) {
	return For(ctx).OrganizationByID.Load(orgID)
}
