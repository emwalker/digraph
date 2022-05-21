package loaders

//go:generate go run github.com/vektah/dataloaden RepositoryLoader string "*github.com/emwalker/digraph/golang/internal/models.Repository"

import (
	"context"
	"log"
	"time"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

type repositoryFetcher func(ids []string) ([]*models.Repository, []error)

func fetchRepositoriesFromDB(ctx context.Context, c *config) repositoryFetcher {
	return func(ids []string) ([]*models.Repository, []error) {
		log.Printf("loaders: fetching repository ids %v", ids)
		repos, err := models.Repositories(
			qm.WhereIn("id in ?", convertIds(ids)...),
		).All(ctx, c.exec)

		if err != nil {
			return nil, []error{err}
		}

		lookup := make(map[string]*models.Repository, len(ids))
		for _, repo := range repos {
			lookup[repo.ID] = repo
		}

		var sorted []*models.Repository
		for _, id := range ids {
			sorted = append(sorted, lookup[id])
		}

		return sorted, nil
	}
}

func newRepositoryLoader(ctx context.Context, exec boil.ContextExecutor, wait time.Duration) *RepositoryLoader {
	return NewRepositoryLoader(RepositoryLoaderConfig{
		Wait:     2 * time.Millisecond,
		MaxBatch: 100,
		Fetch:    fetchRepositoriesFromDB(ctx, &config{exec}),
	})
}
