package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func (m mutator) createLink() (*models.Link, func()) {
	payload1, err := m.resolver.UpsertLink(m.ctx, models.UpsertLinkInput{
		AddParentTopicIds: []string{},
		OrganizationID:    orgId,
		URL:               "https://gnusto.blog",
	})
	if err != nil {
		m.t.Fatal(err)
	}

	link := payload1.LinkEdge.Node

	cleanup := func() {
		count, err := link.Delete(m.ctx, testDB)
		if err != nil {
			m.t.Fatal(err)
		}

		if count != int64(1) {
			m.t.Fatal("Expected at least one updated record")
		}
	}

	return &link, cleanup
}

func TestUpsertLink(t *testing.T) {
	m := newMutator(t)

	topic, err := models.Topics().One(m.ctx, testDB)
	assert.Nil(t, err)

	input := models.UpsertLinkInput{
		AddParentTopicIds: []string{topic.ID},
		OrganizationID:    orgId,
		URL:               "https://gnusto.blog",
	}

	countBefore, err := models.Links().Count(m.ctx, m.db)
	payload1, err := m.resolver.UpsertLink(m.ctx, input)
	if err != nil {
		m.t.Fatal(err)
	}

	link := payload1.LinkEdge.Node

	defer func() {
		count, err := link.Delete(m.ctx, m.db)
		if err != nil {
			t.Fatal(err)
		}
		if count != int64(1) {
			t.Fatal("Expected at least one record to be deleted")
		}
	}()

	countAfter, _ := models.Links().Count(m.ctx, m.db)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should increase")

	assert.NotNil(t, payload1)
	assert.Equal(t, input.URL, payload1.LinkEdge.Node.URL)

	topics, err := link.ParentTopics().All(m.ctx, m.db)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(topics))

	_, err = m.resolver.UpsertLink(m.ctx, input)
	assert.Nil(t, err)
	countAfter, _ = models.Links().Count(m.ctx, m.db)
	assert.Equal(t, countBefore+1, countAfter, "The number of links should stay the same")
}

func TestUpdateParentTopics(t *testing.T) {
	m := newMutator(t)

	link, cleanup := m.createLink()
	defer cleanup()

	topics, err := link.ParentTopics().All(m.ctx, m.db)
	assert.Equal(t, 0, len(topics))

	addTopics, err := models.Topics(qm.Limit(3)).All(m.ctx, m.db)
	assert.Nil(t, err)
	var topicIds []string

	for _, topic := range addTopics {
		topicIds = append(topicIds, topic.ID)
	}

	payload2, err := m.resolver.UpdateLinkTopics(m.ctx, models.UpdateLinkTopicsInput{
		LinkID:         link.ID,
		ParentTopicIds: topicIds,
	})
	assert.Nil(t, err)
	assert.NotNil(t, payload2)

	parentTopics, err := link.ParentTopics().All(m.ctx, m.db)
	assert.Nil(t, err)
	assert.NotZero(t, len(parentTopics))
}

func TestAvailableTopics(t *testing.T) {
	m := newMutator(t)

	link, cleanup := m.createLink()
	defer cleanup()

	query := (&resolvers.Resolver{DB: m.db}).Link()

	connection, err := query.AvailableParentTopics(m.ctx, link, nil, nil, nil, nil)
	if assert.Nil(t, err) {
		assert.True(t, len(connection.Edges) > 0)
	}
}
