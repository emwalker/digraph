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
	r, ctx := startMutationTest(t, testDB)

	input := models.UpsertLinkInput{
		AddTopicIds:    []string{},
		OrganizationID: orgId,
		Title:          "Gnusto's blog",
		URL:            "https://gnusto.blog",
	}

	countBefore, err := models.Links().Count(ctx, testDB)
	payload1, err := r.UpsertLink(ctx, input)
	assert.Nil(t, err)
	countAfter, _ := models.Links().Count(ctx, testDB)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should increase")

	assert.NotNil(t, payload1)
	assert.Equal(t, payload1.LinkEdge.Node.URL, input.URL+"/")

	payload2, err := r.UpsertLink(ctx, input)
	assert.Nil(t, err)
	countAfter, _ = models.Links().Count(ctx, testDB)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should stay the same")

	count, err := payload2.LinkEdge.Node.Delete(ctx, testDB)
	if assert.Nil(t, err) {
		assert.Equal(t, int64(1), count)
	}
}
