package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/stretchr/testify/assert"
)

func TestUpsertTopic(t *testing.T) {
	m := newMutator(t)

	t1, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	parent, err := t1.ParentTopics().One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if parent == nil {
		t.Fatal("The topic should have a parent topic")
	}

	// It does not create a second topic with the same name within the specified organization.
	input := models.UpsertTopicInput{
		Name:           "Agriculture",
		OrganizationID: orgId,
	}

	payload, err := m.resolver.UpsertTopic(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}
	t2 := payload.TopicEdge.Node

	if t1.ID != t2.ID {
		t.Fatal("Another topic with the same name was created")
	}

	if len(payload.Alerts) < 1 {
		t.Fatal("UpsertTopic should add an alert about this being a duplicate")
	}

	if payload.Alerts[0].Type != models.AlertTypeSuccess {
		t.Fatal("Expected a success alert")
	}
}

func TestUpsertTopicDoesNotAllowCycles(t *testing.T) {
	m := newMutator(t)

	t1, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	t2, cleanup := m.createTopic("Husbandry")
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)

	input := models.UpsertTopicInput{
		Name:           "Agriculture",
		OrganizationID: orgId,
		TopicIds:       []string{t2.ID},
	}

	payload, err := m.resolver.UpsertTopic(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload.Alerts) == 0 {
		t.Fatal("UpsertTopic should add an alert about not being able to create a cycle")
	}

	if payload.Alerts[0].Type != models.AlertTypeWarn {
		t.Fatal("Expected a warning")
	}

	if payload.TopicEdge != nil {
		t.Fatal("Expected topic not to be upserted")
	}
}

func TestUpsertTopicDoesNotAllowLinks(t *testing.T) {
	m := newMutator(t)

	input := models.UpsertTopicInput{
		Name:           "http://gnusto.blog",
		OrganizationID: orgId,
	}

	payload, err := m.resolver.UpsertTopic(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if payload.TopicEdge != nil {
		t.Fatal("UpsertTopic should not create a topic from a link")
	}

	if len(payload.Alerts) < 1 {
		t.Fatal("Expected an alert")
	}
}

func TestUpdateParentTopicsDoesNotAllowCycles(t *testing.T) {
	m := newMutator(t)

	t1, cleanup := m.createTopic("Grandparent")
	defer cleanup()

	t2, cleanup := m.createTopic("Parent")
	defer cleanup()

	t3, cleanup := m.createTopic("Child")
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)
	m.addParentTopicToTopic(t3, t2)

	input := models.UpdateTopicParentTopicsInput{
		TopicID:        t1.ID,
		ParentTopicIds: []string{t3.ID},
	}

	payload, err := m.resolver.UpdateTopicParentTopics(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload.Alerts) < 1 {
		t.Fatal("Expected an alert that a topic could not be added as a parent")
	}
}

func TestUpdateTopic(t *testing.T) {
	m := newMutator(t)

	topic, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	assert.Equal(t, "Agriculture", topic.Name)

	var err error
	desc := "Cultivating"

	input := models.UpdateTopicInput{
		Name:           "Agricultures",
		Description:    &desc,
		OrganizationID: orgId,
		ID:             topic.ID,
	}

	p2, err := m.resolver.UpdateTopic(m.ctx, input)

	if !assert.Nil(t, err) {
		return
	}

	assert.Equal(t, topic.ID, p2.Topic.ID)

	topic = &p2.Topic
	err = topic.Reload(m.ctx, m.db)
	assert.Nil(t, err)
	assert.Equal(t, "Agricultures", topic.Name)
}

func TestTopicParentTopics(t *testing.T) {
	m := newMutator(t)

	topic1, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	topic2, cleanup := m.createTopic("Crop rotation")
	defer cleanup()

	parentTopics, err := topic2.ParentTopics().All(m.ctx, m.db)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(parentTopics))

	m.addParentTopicToTopic(topic2, topic1)

	if parentTopics, err = topic2.ParentTopics().All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}
	assert.Equal(t, 2, len(parentTopics))
}

func TestSearchChildTopics(t *testing.T) {
	m := newMutator(t)

	topic, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	childTopic, cleanup := m.createTopic("Crop rotation")
	defer cleanup()

	m.addParentTopicToTopic(childTopic, topic)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "",
			Count:        1,
		},
		{
			Name:         "When there is a full match",
			SearchString: "crop rotation",
			Count:        1,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "crop rota",
			Count:        1,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "rotation",
			Count:        0, // Maybe later
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := (&resolvers.Resolver{DB: testDB}).Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.ChildTopics(m.ctx, topic, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); td.Count != count {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}

func TestSearchLinksInTopic(t *testing.T) {
	m := newMutator(t)

	topic, cleanup := m.createTopic("News organizations")
	defer cleanup()

	link, cleanup := m.createLink("New York Times", "https://www.nytimes.com")
	defer cleanup()

	m.addParentTopicToLink(link, topic)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "",
			Count:        1,
		},
		{
			Name:         "When there is a full match",
			SearchString: "New York Times",
			Count:        1,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "New Yor",
			Count:        1,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "York Times",
			Count:        0, // Maybe later
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := (&resolvers.Resolver{DB: testDB}).Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.Links(m.ctx, topic, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); td.Count != count {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}
