package resolvers

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestLinks(t *testing.T) {
	testDB = newTestDb(t)
	defer testDB.Close()

	t.Run("upsertLink", upsertLinkTest)
	t.Run("updateParentTopics", updateParentTopics)
	t.Run("availableTopics", availableTopics)
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
	link := payload1.LinkEdge.Node

	defer func() {
		count, err := link.Delete(ctx, testDB)
		if assert.Nil(t, err) {
			assert.Equal(t, int64(1), count)
		}
	}()

	countAfter, _ := models.Links().Count(ctx, testDB)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should increase")

	assert.NotNil(t, payload1)
	assert.Equal(t, payload1.LinkEdge.Node.URL, input.URL+"/")

	_, err = r.UpsertLink(ctx, input)
	assert.Nil(t, err)
	countAfter, _ = models.Links().Count(ctx, testDB)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should stay the same")
}

func createLink(t *testing.T, ctx context.Context, r models.MutationResolver) (*models.Link, func()) {
	payload1, err := r.UpsertLink(ctx, models.UpsertLinkInput{
		AddTopicIds:    []string{},
		OrganizationID: orgId,
		Title:          "Gnusto's blog",
		URL:            "https://gnusto.blog",
	})
	assert.Nil(t, err)

	link := payload1.LinkEdge.Node

	cleanup := func() {
		count, err := link.Delete(ctx, testDB)
		if assert.Nil(t, err) {
			assert.Equal(t, int64(1), count)
		}
	}

	return &link, cleanup
}

func updateParentTopics(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	link, cleanup := createLink(t, ctx, r)
	defer cleanup()

	topics, err := link.ParentTopics().All(ctx, testDB)
	assert.Equal(t, 0, len(topics))

	addTopics, err := models.Topics(qm.Limit(3)).All(ctx, testDB)
	assert.Nil(t, err)
	var topicIds []string

	for _, topic := range addTopics {
		topicIds = append(topicIds, topic.ID)
	}

	payload2, err := r.UpdateLinkTopics(ctx, models.UpdateLinkTopicsInput{
		LinkID:         link.ID,
		ParentTopicIds: topicIds,
	})
	assert.Nil(t, err)
	assert.NotNil(t, payload2)

	parentTopics, err := link.ParentTopics().All(ctx, testDB)
	assert.Nil(t, err)
	assert.NotZero(t, len(parentTopics))
}

func availableTopics(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	link, cleanup := createLink(t, ctx, r)
	defer cleanup()

	query := (&resolvers.Resolver{DB: testDB}).Link()

	connection, err := query.AvailableParentTopics(ctx, link, nil, nil, nil, nil)
	if assert.Nil(t, err) {
		assert.True(t, len(connection.Edges) > 0)
	}
}
