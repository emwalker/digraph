package services_test

import (
	"context"
	"database/sql"
	"log"
	"os"
	"testing"

	"github.com/emwalker/digraph/models"
	_ "github.com/lib/pq"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

const orgId = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

var (
	testActor   *models.User
	testDB      *sql.DB
	defaultRepo *models.Repository
)

func TestMain(m *testing.M) {
	testDB = newTestDb()
	defer testDB.Close()

	var err error
	ctx := context.Background()

	scope := models.Repositories(qm.Where("organization_id = ? and system", orgId))
	if defaultRepo, err = scope.One(ctx, testDB); err != nil {
		panic(err)
	}

	if testActor, err = models.Users().One(ctx, testDB); err != nil {
		panic(err)
	}

	os.Exit(m.Run())
}

func newTestDb() *sql.DB {
	var err error
	if testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable"); err != nil {
		log.Fatal("Unable to connect to the database", err)
	}
	return testDB
}
