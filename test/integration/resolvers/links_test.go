package resolvers

import (
	"context"
	"database/sql"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
)

var testDB *sql.DB
var err error

const orgId = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

func TestMain(t *testing.T) {
	testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
	assert.NoError(t, err)
	defer testDB.Close()

	t.Run("UpsertLinks", upsertLinksTest)
}

func upsertLinksTest(t *testing.T) {
	tx, err := testDB.Begin()
	defer tx.Rollback()

	input := models.UpsertLinkInput{
		AddTopicIds:    []string{},
		OrganizationID: orgId,
		Title:          "Gnusto's blog",
		URL:            "https://gnusto.blog",
	}

	r := &resolvers.MutationResolver{&resolvers.Resolver{DB: testDB, Tx: tx}}

	countBefore, _ := models.Links().Count(context.Background(), tx)
	var payload *models.UpsertLinkPayload
	payload, err = r.UpsertLink(context.Background(), input)
	assert.Nil(t, err)
	countAfter, _ := models.Links().Count(context.Background(), tx)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should increase")

	assert.NotNil(t, payload)
	assert.Equal(t, payload.LinkEdge.Node.URL, input.URL+"/")

	_, err = r.UpsertLink(context.Background(), input)
	assert.Nil(t, err)
	countAfter, _ = models.Links().Count(context.Background(), tx)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should stay the same")
}
