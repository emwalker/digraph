package resolvers

import (
	"context"
	"database/sql"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/emwalker/digraph/resolvers/pageinfo"
	"github.com/stretchr/testify/assert"
)

const orgId = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

var testDB *sql.DB

type testFetcher struct{}

func newTestDb(t *testing.T) *sql.DB {
	var err error
	testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
	assert.Nil(t, err, "Unable to connect to the database")
	return testDB
}

func startMutationTest(t *testing.T, db *sql.DB) (models.MutationResolver, context.Context) {
	resolver := &resolvers.MutationResolver{
		&resolvers.Resolver{DB: db},
		&testFetcher{},
	}
	return resolver, context.Background()
}

func (f *testFetcher) FetchPage(url string) (*pageinfo.PageInfo, error) {
	title := "Gnusto's blog"
	return &pageinfo.PageInfo{
		URL:   url,
		Title: &title,
	}, nil
}
