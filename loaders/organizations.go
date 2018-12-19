package loaders

//go:generate dataloaden -keys string github.com/emwalker/digraph/models.Organization

import (
	"context"
	"database/sql"
	"log"
	"time"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// OrganizationLoaderKey is the key under which the topic loader is stored in the session.
const OrganizationLoaderKey = "organizationLoader"

func fetchOrganizationsFromDB(
	ctx context.Context, c *config,
) func(ids []string) ([]*models.Organization, []error) {
	return func(ids []string) ([]*models.Organization, []error) {
		log.Printf("Fetching organization ids %v", ids)
		orgs, err := models.Organizations(
			qm.WhereIn("id in ?", convertIds(ids)...),
		).All(ctx, c.db)

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

// NewOrganizationLoader returns a loader that can be used to batch load organizations.
func NewOrganizationLoader(ctx context.Context, db *sql.DB) *OrganizationLoader {
	return &OrganizationLoader{
		maxBatch: 100,
		wait:     1 * time.Millisecond,
		fetch:    fetchOrganizationsFromDB(ctx, &config{db}),
	}
}
