package services_test

import (
	"context"
	"database/sql"
	"log"
	"os"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	_ "github.com/lib/pq"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

var (
	testActor   *models.User
	testDB      *sql.DB
	defaultRepo *models.Repository
)

type testFetcher struct{}

func (f testFetcher) FetchPage(url string) (*pageinfo.PageInfo, error) {
	title := "Title"
	return &pageinfo.PageInfo{url, &title}, nil
}

func TestMain(m *testing.M) {
	testDB = newTestDb()
	defer testDB.Close()

	var err error
	ctx := context.Background()

	scope := models.Repositories(qm.Where("organization_id = ? and system", services.PublicOrgID))
	if defaultRepo, err = scope.One(ctx, testDB); err != nil {
		panic(err)
	}

	if testActor, err = models.Users().One(ctx, testDB); err != nil {
		panic(err)
	}

	services.Fetcher = testFetcher{}

	os.Exit(m.Run())
}

func newTestDb() *sql.DB {
	var err error
	if testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable"); err != nil {
		log.Fatal("Unable to connect to the database", err)
	}
	return testDB
}
