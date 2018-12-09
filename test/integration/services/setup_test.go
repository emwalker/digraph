package services_test

import (
	"log"
	"os"

	"database/sql"
	_ "github.com/lib/pq"
	"testing"
)

const orgId = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

var testDB *sql.DB

func TestMain(m *testing.M) {
	testDB = newTestDb()
	defer testDB.Close()
	os.Exit(m.Run())
}

func newTestDb() *sql.DB {
	var err error
	if testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable"); err != nil {
		log.Fatal("Unable to connect to the database", err)
	}
	return testDB
}
