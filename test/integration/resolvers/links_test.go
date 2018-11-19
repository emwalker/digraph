package resolvers

import (
	"testing"

	"github.com/emwalker/digraph/models"
	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
)

func TestLinks(t *testing.T) {
	testDB = newTestDb(t)
	defer testDB.Close()

	t.Run("upsertLink", upsertLinkTest)
}

func upsertLinkTest(t *testing.T) {
	r, ctx, tx := startMutationTest(t, testDB)
	defer tx.Rollback()

	input := models.UpsertLinkInput{
		AddTopicIds:    []string{},
		OrganizationID: orgId,
		Title:          "Gnusto's blog",
		URL:            "https://gnusto.blog",
	}

	var err error
	var payload *models.UpsertLinkPayload

	countBefore, _ := models.Links().Count(ctx, tx)
	payload, err = r.UpsertLink(ctx, input)
	assert.Nil(t, err)
	countAfter, _ := models.Links().Count(ctx, tx)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should increase")

	assert.NotNil(t, payload)
	assert.Equal(t, payload.LinkEdge.Node.URL, input.URL+"/")

	_, err = r.UpsertLink(ctx, input)
	assert.Nil(t, err)
	countAfter, _ = models.Links().Count(ctx, tx)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should stay the same")
}
