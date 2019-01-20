package resolvers_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/internal/models"
	"github.com/emwalker/digraph/internal/resolvers"
	"github.com/emwalker/digraph/internal/services"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertTopic(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
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
		Name:              "Agriculture",
		OrganizationLogin: testActor.Login,
		RepositoryName:    repoName,
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
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
	defer cleanup()

	t2, cleanup := m.createTopic(testActor.Login, repoName, "Husbandry")
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)

	input := models.UpsertTopicInput{
		Name:              "Agriculture",
		OrganizationLogin: testActor.Login,
		RepositoryName:    repoName,
		TopicIds:          []string{t2.ID},
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
	m := newMutator(t, testActor)

	input := models.UpsertTopicInput{
		Name:              "http://gnusto.blog",
		OrganizationLogin: testActor.Login,
		RepositoryName:    m.defaultRepo().Name,
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
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testActor.Login, repoName, "Grandparent")
	defer cleanup()

	t2, cleanup := m.createTopic(testActor.Login, repoName, "Parent")
	defer cleanup()

	t3, cleanup := m.createTopic(testActor.Login, repoName, "Child")
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
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "Agriculture")
	defer cleanup()

	assert.Equal(t, "Agriculture", topic.Name)

	var err error
	desc := "Cultivating"

	input := models.UpdateTopicInput{
		Name:        "Agricultura",
		Description: &desc,
		ID:          topic.ID,
	}

	p2, err := m.resolver.UpdateTopic(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if topic.ID != p2.Topic.ID {
		t.Fatal("Expected the topics to be the same")
	}

	topic = &p2.Topic
	if err = topic.Reload(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if topic.Name != "Agricultura" {
		t.Fatal("Expected the name of the topic to be updated")
	}
}

func TestPreventingUpdateTopicFromCreatingADuplicate(t *testing.T) {
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "Agriculture")
	defer cleanup()

	if topic.Name != "Agriculture" {
		t.Fatalf("Expected new topic to have the name 'Agriculture': %s", topic.Name)
	}

	_, cleanup = m.createTopic(testActor.Login, m.defaultRepo().Name, "Agricultura")
	defer cleanup()

	// Try to give our first topic the same name as the second topic
	input := models.UpdateTopicInput{
		Name: "Agricultura",
		ID:   topic.ID,
	}

	payload, err := m.resolver.UpdateTopic(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload.Alerts) < 1 {
		t.Fatal("Expected an alert")
	}
}

func TestTopicParentTopics(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	topic1, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
	defer cleanup()

	topic2, cleanup := m.createTopic(testActor.Login, repoName, "Crop rotation")
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
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
	defer cleanup()

	childTopic, cleanup := m.createTopic(testActor.Login, repoName, "Crop rotation")
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
			Count:        1,
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := resolvers.New(testDB, testActor).Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.ChildTopics(m.ctx, topic, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			var count int
			if count = len(conn.Edges); td.Count != count {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}

func TestSearchLinksInTopic(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "News organizations")
	defer cleanup()

	link, cleanup := m.createLink(testActor.Login, repoName, "New York Times", "https://www.nytimes.com")
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
			Count:        1,
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := resolvers.New(testDB, testActor).Topic()

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

func TestSearchInTopic(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testActor.Login, repoName, "News organizations")
	defer cleanup()

	l1, cleanup := m.createLink(testActor.Login, repoName, "News", "https://en.wikipedia.org/wiki/News")
	defer cleanup()
	m.addParentTopicToLink(l1, t1)

	t2, cleanup := m.createTopic(testActor.Login, repoName, "New York Times")
	defer cleanup()
	m.addParentTopicToTopic(t2, t1)

	l2, cleanup := m.createLink(testActor.Login, repoName, "New York Times", "https://www.nytimes.com")
	defer cleanup()
	m.addParentTopicToLink(l2, t2)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
	}{
		{
			Name:         "Everything is returned when an empty string is provided",
			SearchString: "",
			Count:        4,
		},
		{
			Name:         "Links directly under the topic are returned",
			SearchString: "News",
			Count:        2,
		},
		{
			Name:         "Descendant links and topics are returned",
			SearchString: "New York Times",
			Count:        2,
		},
		{
			Name:         "Prefix matches work",
			SearchString: "New Yor",
			Count:        2,
		},
		{
			Name:         "Suffix matches work",
			SearchString: "York Times",
			Count:        2,
		},
		{
			Name:         "No results are returned when there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := resolvers.New(testDB, testActor).Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.Search(m.ctx, t1, td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			var count int
			if count = len(conn.Edges); td.Count != count {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}

			if count > 0 {
				topic, ok := conn.Edges[0].Node.(models.TopicValue)
				if !ok {
					t.Fatalf("Unable to cast %#v to a topic", conn.Edges[0].Node)
				}

				if topic.R == nil || topic.R.ParentTopics == nil {
					t.Fatal("Expected parent topics to be preloaded")
				}
			}
		})
	}
}

func TestRootTopicIncludedInResults(t *testing.T) {
	m := newMutator(t, testActor)
	ctx := context.Background()

	var err error
	var root *models.TopicValue
	if root, err = m.defaultRepo().RootTopic(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "News organizations")
	defer cleanup()
	m.addParentTopicToTopic(topic, root)

	topicResolver := resolvers.New(testDB, testActor).Topic()

	var conn models.SearchResultItemConnection
	if conn, err = topicResolver.Search(ctx, root, root.Name, nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatal("Expected a result")
	}

	resultTopicIds := make(map[string]bool)
	for _, edge := range conn.Edges {
		if topic, ok := edge.Node.(models.TopicValue); ok {
			resultTopicIds[topic.ID] = true
		}
	}

	if len(resultTopicIds) < 1 {
		t.Fatal("Expected at least one topic")
	}

	if _, ok := resultTopicIds[root.ID]; !ok {
		t.Fatal("Expected root topic to show up in results")
	}
}

func TestParentTopicPreloading(t *testing.T) {
	r := resolvers.New(testDB, testActor).Topic()
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testActor.Login, repoName, "News organizations")
	defer cleanup()

	t2, cleanup := m.createTopic(testActor.Login, repoName, "New York Times")
	defer cleanup()
	m.addParentTopicToTopic(t2, t1)

	var err error
	var connection models.TopicConnection

	if connection, err = r.ChildTopics(m.ctx, t1, nil, nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected at least one child topic")
	}

	child := connection.Edges[0].Node
	if child.R == nil || child.R.ParentTopics == nil {
		t.Fatal("Parent topics not preloaded")
	}
}

func TestAvailableTopicsForTopicsFromOtherRepos(t *testing.T) {
	m := newMutator(t, testActor)
	s := services.New(testDB, testActor)

	org1, err := models.Organizations(qm.Where("login = ?", testActor.Login)).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	org2, err := models.Organizations(qm.Where("login = ?", "wiki")).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	r1, err := s.CreateRepository(m.ctx, org1, "r1", testActor, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r1.Cleanup()

	r2, err := s.CreateRepository(m.ctx, org2, "r2", testActor, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r2.Cleanup()

	_, cleanup := m.createTopic(testActor.Login, r1.Repository.Name, "Topic 1")
	defer cleanup()

	topic2, cleanup := m.createTopic("wiki", r2.Repository.Name, "Topic 2")
	defer cleanup()

	query := resolvers.New(m.db, testActor).Topic()

	conn, err := query.AvailableParentTopics(m.ctx, topic2, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 2 {
		t.Fatal("Expected at least one topic edge")
	}
}

func TestDeleteTopic(t *testing.T) {
	m := newMutator(t, testActor)

	topic, _ := m.createTopic(testActor.Login, m.defaultRepo().Name, "A new topic")

	payload, err := m.resolver.DeleteTopic(m.ctx, models.DeleteTopicInput{
		TopicID: topic.ID,
	})
	if err != nil {
		t.Fatal(err)
	}

	if payload == nil {
		t.Fatal("Expected a payload")
	}

	count, err := models.Topics(qm.Where("id = ?", topic.ID)).Count(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatal("Failed to delete topic")
	}
}
