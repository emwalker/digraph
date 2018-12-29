package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertLink(t *testing.T) {
	m := newMutator(t, testActor)

	topic, err := models.Topics().One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	input := models.UpsertLinkInput{
		AddParentTopicIds: []string{topic.ID},
		OrganizationLogin: testActor.Login,
		RepositoryName:    m.defaultRepo().Name,
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
	if countAfter != countBefore+1 {
		t.Fatal("The number of links should increase")
	}

	if input.URL != payload1.LinkEdge.Node.URL {
		t.Fatal("Unexpected url", payload1.LinkEdge.Node.URL)
	}

	topics, err := link.ParentTopics().All(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if len(topics) != 1 {
		t.Fatal("Expected link to have a topic")
	}

	payload2, err := m.resolver.UpsertLink(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload2.Alerts) < 1 {
		t.Fatal("Expected an alert")
	}

	countAfter, _ = models.Links().Count(m.ctx, m.db)
	if countAfter != countBefore+1 {
		t.Fatal("The number of links should stay the same")
	}
}

func TestUpdateParentTopics(t *testing.T) {
	m := newMutator(t, testActor)

	link, cleanup := m.createLink("Gnusto's Blog", "https://gnusto.blog")
	defer cleanup()

	var topics []*models.Topic
	var err error

	if topics, err = link.ParentTopics().All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if len(topics) > 1 {
		t.Fatal("Expected to find only a single topic")
	}

	var addTopics []*models.Topic
	if addTopics, err = models.Topics(qm.Limit(3)).All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	var topicIds []string
	for _, topic := range addTopics {
		topicIds = append(topicIds, topic.ID)
	}

	payload2, err := m.resolver.UpdateLinkTopics(m.ctx, models.UpdateLinkTopicsInput{
		LinkID:         link.ID,
		ParentTopicIds: topicIds,
	})
	if err != nil {
		t.Fatal(err)
	}

	if payload2 == nil {
		t.Fatal("Expected a non-nil result for payload2")
	}

	var parentTopics []*models.Topic
	if parentTopics, err = link.ParentTopics().All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if len(parentTopics) < 1 {
		t.Fatal("Expected at least one topic")
	}
}

func TestAvailableTopics(t *testing.T) {
	m := newMutator(t, testActor)

	_, cleanup := m.createTopic("Something")
	defer cleanup()

	link, cleanup := m.createLink("Gnusto's Blog", "https://gnusto.blog")
	defer cleanup()

	query := (&resolvers.Resolver{DB: m.db}).Link()

	connection, err := query.AvailableParentTopics(m.ctx, link, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected at least one topic edge")
	}
}
