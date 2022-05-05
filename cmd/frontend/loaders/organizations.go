package loaders

//go:generate go run github.com/vektah/dataloaden OrganizationLoader string "*github.com/emwalker/digraph/cmd/frontend/models.Organization"

import (
	"context"
	"log"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// OrganizationLoaderKey is the key under which the topic loader is stored in the session.
const OrganizationLoaderKey = "organizationLoader"

type organizationFetcher func(ids []string) ([]*models.Organization, []error)

func fetchOrganizationsFromDB(ctx context.Context, c *config) organizationFetcher {
	return func(ids []string) ([]*models.Organization, []error) {
		log.Printf("Fetching organization ids %v", ids)
		orgs, err := models.Organizations(
			qm.WhereIn("id in ?", convertIds(ids)...),
		).All(ctx, c.exec)

		if err != nil {
			return nil, []error{err}
		}

		lookup := make(map[string]*models.Organization, len(ids))
		for _, org := range orgs {
			lookup[org.ID] = org
		}

		var sorted []*models.Organization
		for _, id := range ids {
			sorted = append(sorted, lookup[id])
		}

		return sorted, nil
	}
}

func newOrganizationLoader(ctx context.Context, exec boil.ContextExecutor, wait time.Duration) *OrganizationLoader {
	return NewOrganizationLoader(OrganizationLoaderConfig{
		MaxBatch: 1000,
		Wait:     wait,
		Fetch:    fetchOrganizationsFromDB(ctx, &config{exec}),
	})
}
