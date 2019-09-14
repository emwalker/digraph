package resolvers_test

import (
	"context"
	"reflect"
	"strings"
	"testing"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertTopic(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
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
		OrganizationLogin: testViewer.Login,
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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
	defer cleanup()

	t2, cleanup := m.createTopic(testViewer.Login, repoName, "Husbandry")
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)

	input := models.UpsertTopicInput{
		Name:              "Agriculture",
		OrganizationLogin: testViewer.Login,
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
	m := newMutator(t, testViewer)

	input := models.UpsertTopicInput{
		Name:              "http://gnusto.blog",
		OrganizationLogin: testViewer.Login,
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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "Grandparent")
	defer cleanup()

	t2, cleanup := m.createTopic(testViewer.Login, repoName, "Parent")
	defer cleanup()

	t3, cleanup := m.createTopic(testViewer.Login, repoName, "Child")
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

func TestUpdateTopicPreventsOwnTopic(t *testing.T) {
	m := newMutator(t, testViewer)

	topic, cleanup := m.createTopic(testViewer.Login, m.defaultRepo().Name, "Agriculture")
	defer cleanup()

	if topic.Name != "Agriculture" {
		t.Fatal("Expected the name 'Agriculture'")
	}

	input := models.UpdateTopicParentTopicsInput{
		TopicID:        topic.ID,
		ParentTopicIds: []string{topic.ID},
	}

	result, err := m.resolver.UpdateTopicParentTopics(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(result.Alerts) < 1 {
		t.Fatal("Expected an alert")
	}
}

func TestUpdateTopic(t *testing.T) {
	m := newMutator(t, testViewer)

	topic, cleanup := m.createTopic(testViewer.Login, m.defaultRepo().Name, "Agriculture")
	defer cleanup()

	if topic.Name != "Agriculture" {
		t.Fatal("Expected the name 'Agriculture'")
	}

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

	topic = p2.Topic
	if err = topic.Reload(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if topic.Name != "Agricultura" {
		t.Fatal("Expected the name of the topic to be updated")
	}
}

func TestPreventingUpdateTopicFromCreatingADuplicate(t *testing.T) {
	m := newMutator(t, testViewer)

	topic, cleanup := m.createTopic(testViewer.Login, m.defaultRepo().Name, "Agriculture")
	defer cleanup()

	if topic.Name != "Agriculture" {
		t.Fatalf("Expected new topic to have the name 'Agriculture': %s", topic.Name)
	}

	_, cleanup = m.createTopic(testViewer.Login, m.defaultRepo().Name, "Agricultura")
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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic1, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
	defer cleanup()

	topic2, cleanup := m.createTopic(testViewer.Login, repoName, "Crop rotation")
	defer cleanup()

	parentTopics, err := topic2.ParentTopics().All(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if len(parentTopics) != 1 {
		t.Fatal("Expected one parent topic")
	}

	m.addParentTopicToTopic(topic2, topic1)

	if parentTopics, err = topic2.ParentTopics().All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if len(parentTopics) != 2 {
		t.Fatal("Expected 2 parent topics")
	}
}

func TestChildTopicsDefaultOrdering(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
	defer cleanup()

	childTopic1, cleanup := m.createTopic(testViewer.Login, repoName, "A")
	defer cleanup()

	childTopic2, cleanup := m.createTopic(testViewer.Login, repoName, "B")
	defer cleanup()

	m.addParentTopicToTopic(childTopic1, topic)
	m.addParentTopicToTopic(childTopic2, topic)

	input := models.UpdateSynonymsInput{
		Synonyms: []*models.SynonymInput{
			{Locale: "en", Name: "C"},
			{Locale: "en", Name: "A"},
		},
		TopicID: childTopic1.ID,
	}

	resolver := rootResolver.Mutation()
	if _, err := resolver.UpdateSynonyms(m.ctx, input); err != nil {
		t.Fatal(err)
	}

	topicResolver := rootResolver.Topic()
	conn, err := topicResolver.ChildTopics(m.ctx, topic, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 2 {
		t.Fatalf("Expected two results: %#v", conn.Edges)
	}

	prevName := ""

	for _, edge := range conn.Edges {
		synonyms, err := edge.Node.SynonymList()
		if err != nil {
			t.Fatal(err)
		}

		currName, ok := synonyms.NameForLocale("en")
		if !ok {
			t.Fatal("There was a problem fetching the display name")
		}

		if prevName != "" && currName <= prevName {
			t.Fatalf("Expected %s to come before %s", currName, prevName)
		}

		prevName = currName
	}
}

func TestSearchChildTopics(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
	defer cleanup()

	childTopic, cleanup := m.createTopic(testViewer.Login, repoName, "Crop rotation")
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

	topicResolver := rootResolver.Topic()

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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "News organizations")
	defer cleanup()

	link, cleanup := m.createLink(testViewer.Login, repoName, "New York Timely", "https://www.nytimely.com")
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
			SearchString: "New York Timely",
			Count:        1,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "New Yor",
			Count:        1,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "York Timely",
			Count:        1,
		},
		{
			Name:         "When there is no match",
			SearchString: "astrogomy",
			Count:        0,
		},
		{
			Name:         "When the search matches the url",
			SearchString: "nytimely",
			Count:        1,
		},
	}

	topicResolver := rootResolver.Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.Links(m.ctx, topic, nil, nil, nil, nil, &td.SearchString, nil, nil)
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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "News organizations")
	defer cleanup()

	l1, cleanup := m.createLink(testViewer.Login, repoName, "News", "https://en.wikipedia.org/wiki/News")
	defer cleanup()
	m.addParentTopicToLink(l1, t1)

	t2, cleanup := m.createTopic(testViewer.Login, repoName, "New York Times")
	defer cleanup()
	m.addParentTopicToTopic(t2, t1)

	l2, cleanup := m.createLink(testViewer.Login, repoName, "New York Times", "https://www.nytimely.com")
	defer cleanup()
	m.addParentTopicToLink(l2, t2)

	cases := []struct {
		name         string
		searchString string
		count        int
	}{
		{
			name:         "Everything is returned when an empty string is provided",
			searchString: "",
			count:        4,
		},
		{
			name:         "Links directly under the topic are returned",
			searchString: "News",
			count:        2,
		},
		{
			name:         "Descendant links and topics are returned",
			searchString: "New York Times",
			count:        2,
		},
		{
			name:         "Prefix matches work",
			searchString: "New Yor",
			count:        2,
		},
		{
			name:         "Suffix matches work",
			searchString: "York Times",
			count:        2,
		},
		{
			name:         "Token matches work",
			searchString: "Times York",
			count:        2,
		},
		{
			name:         "No results are returned when there is no match",
			searchString: "astronomy",
			count:        0,
		},
		{
			name:         "Searches include the urls",
			searchString: "nytimely",
			count:        1,
		},
	}

	topicResolver := rootResolver.Topic()

	for _, td := range cases {
		t.Run(td.name, func(t *testing.T) {
			conn, err := topicResolver.Search(m.ctx, t1, td.searchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			var count int
			if count = len(conn.Edges); td.count != count {
				t.Fatalf("Expected %d results, got %d", td.count, count)
			}

			if count > 0 {
				topic, ok := conn.Edges[0].Node.(models.TopicValue)
				if ok {
					if topic.R == nil || topic.R.ParentTopics == nil {
						t.Fatal("Expected parent topics to be preloaded")
					}
				}
			}
		})
	}
}

func TestRootTopicIncludedInResults(t *testing.T) {
	t.Skip("Fix test flake or delete")

	m := newMutator(t, testViewer)

	var err error
	var root *models.TopicValue

	if root, err = m.defaultRepo().RootTopic(m.ctx, testDB, testViewer.DefaultView()); err != nil {
		t.Fatal(err)
	}
	if root.View == nil {
		t.Fatal("Expected a view")
	}

	topic, cleanup := m.createTopic(testViewer.Login, m.defaultRepo().Name, "News organizations")
	defer cleanup()
	m.addParentTopicToTopic(topic, root)

	topicResolver := rootResolver.Topic()

	var conn *models.SearchResultItemConnection

	if conn, err = topicResolver.Search(m.ctx, root, root.Name, nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatalf("Expected a result, %s", testViewer)
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
		t.Fatalf("Expected root topic to show up in results, %s", testViewer)
	}
}

func TestParentTopicPreloading(t *testing.T) {
	r := rootResolver.Topic()
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "News organizations")
	defer cleanup()

	t2, cleanup := m.createTopic(testViewer.Login, repoName, "New York Times")
	defer cleanup()
	m.addParentTopicToTopic(t2, t1)

	var err error
	var connection *models.TopicConnection

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
	m := newMutator(t, testViewer)
	s := services.New(testDB, testViewer, rootResolver.Fetcher)

	org1, err := models.Organizations(qm.Where("login = ?", testViewer.Login)).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	org2, err := models.Organizations(qm.Where("login = ?", "wiki")).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	r1, err := s.CreateRepository(m.ctx, org1, "r1", testViewer, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r1.Cleanup()

	r2, err := s.CreateRepository(m.ctx, org2, "r2", testViewer, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r2.Cleanup()

	_, cleanup := m.createTopic(testViewer.Login, r1.Repository.Name, "Topic 1")
	defer cleanup()

	topic2, cleanup := m.createTopic("wiki", r2.Repository.Name, "Topic 2")
	defer cleanup()

	query := rootResolver.Topic()

	conn, err := query.AvailableParentTopics(m.ctx, topic2, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 2 {
		t.Fatal("Expected at least one topic edge")
	}
}

func TestAvailableTopicsForTopicWithFilter(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name
	matchingString := "695be58"
	nonMatchingString := "doesn't match"

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "Topic 1")
	defer cleanup()

	t2, cleanup := m.createTopic(testViewer.Login, repoName, matchingString)
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)

	query := rootResolver.Topic()

	cases := []struct {
		name         string
		searchString *string
		count        int
		atLeast      bool
	}{
		{
			name:         "Everything is returned when there is no search string",
			searchString: nil,
			count:        1,
			atLeast:      true,
		},
		{
			name:         "The matching topic is returned if the search string matches it",
			searchString: &matchingString,
			count:        1,
			atLeast:      false,
		},
		{
			name:         "The matching topic is not returned if the search does not match it",
			searchString: &nonMatchingString,
			count:        0,
			atLeast:      false,
		},
	}

	for _, td := range cases {
		t.Run(td.name, func(t *testing.T) {
			conn, err := query.AvailableParentTopics(m.ctx, t1, td.searchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			count := len(conn.Edges)

			if td.atLeast {
				if td.count > count {
					t.Fatalf("Expected at least %d results, got %d", td.count, count)
				}
			} else if td.count != count {
				t.Fatalf("Expected %d results, got %d", td.count, count)
			}
		})
	}
}

func TestAvailableParentTopicsDoesNotIncludeSelf(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name
	matchingString := "695be58"

	t1, cleanup := m.createTopic(testViewer.Login, repoName, "Topic 1")
	defer cleanup()

	t2, cleanup := m.createTopic(testViewer.Login, repoName, matchingString)
	defer cleanup()

	m.addParentTopicToTopic(t2, t1)
	query := rootResolver.Topic()

	conn, err := query.AvailableParentTopics(m.ctx, t2, &matchingString, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	for _, edge := range conn.Edges {
		if edge.Node.Name == matchingString {
			t.Fatalf("Parent topics should not include self: %s", matchingString)
		}
	}
}

func topicsToString(topics []*models.Topic) string {
	summaries := make([]string, len(topics))
	for i, topic := range topics {
		summaries[i] = topic.String()
	}
	return strings.Join(summaries, ", ")
}

func TestDeleteTopic(t *testing.T) {
	mutator := newMutator(t, testViewer)
	repo := mutator.defaultRepo()
	repoName := repo.Name

	ancestorTopic, cleanup := mutator.createTopic(testViewer.Login, repoName, "Ancestor topic")
	defer cleanup()

	parentTopic, _ := mutator.createTopic(testViewer.Login, repoName, "Parent topic")

	childTopic1, cleanup := mutator.createTopic(testViewer.Login, repoName, "Child topic 1")
	defer cleanup()

	childTopic2, cleanup := mutator.createTopic(testViewer.Login, repoName, "Child topic 2")
	defer cleanup()

	childLink1, cleanup := mutator.createLink(testViewer.Login, repoName, "Child link", "https://en.wikipedia.org/wiki/Child_link")
	defer cleanup()

	mutator.addParentTopicToTopic(parentTopic, ancestorTopic)
	mutator.addParentTopicToTopic(childTopic1, parentTopic)
	mutator.addParentTopicToTopic(childTopic2, parentTopic)
	mutator.addParentTopicToLink(childLink1, parentTopic)

	rootTopic, err := repo.RootTopic(mutator.ctx, testDB, testViewer.DefaultView())
	if err != nil {
		t.Fatal(err)
	}

	_, err = queries.Raw(`
	delete from topic_topics
		where parent_id in ($1, $2)
		and child_id in ($3, $4)
	`, services.PublicRootTopicID, rootTopic.ID, childTopic1.ID, childTopic2.ID).Exec(testDB)

	if err != nil {
		t.Fatal(err)
	}

	_, err = queries.Raw(`
	delete from link_topics
		where parent_id in ($1, $2)
		and child_id = $3
	`, services.PublicRootTopicID, rootTopic.ID, childLink1.ID).Exec(testDB)

	if err != nil {
		t.Fatal(err)
	}

	if topics, _ := childTopic1.ParentTopics().All(mutator.ctx, testDB); len(topics) > 1 {
		t.Fatalf("Expected there to be a single parent topic: %s", topicsToString(topics))
	}

	if topics, _ := childTopic2.ParentTopics().All(mutator.ctx, testDB); len(topics) > 1 {
		t.Fatalf("Expected there to be a single parent topic: %s", topicsToString(topics))
	}

	if topics, _ := childLink1.ParentTopics().All(mutator.ctx, testDB); len(topics) > 1 {
		t.Fatalf("Expected there to be a single parent topic: %s", topicsToString(topics))
	}

	payload, err := mutator.resolver.DeleteTopic(mutator.ctx, models.DeleteTopicInput{
		TopicID: parentTopic.ID,
	})
	if err != nil {
		t.Fatal(err)
	}

	if payload == nil {
		t.Fatal("Expected a payload")
	}

	count, err := models.Topics(qm.Where("id = ?", parentTopic.ID)).Count(mutator.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatal("Failed to delete topic")
	}

	newParentTopic, err := childTopic1.ParentTopics().One(mutator.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if newParentTopic.ID != ancestorTopic.ID {
		t.Fatalf("Expected child topic 1 to be placed under the ancestor topic, got: %s", newParentTopic)
	}

	newParentTopic, err = childTopic2.ParentTopics().One(mutator.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if newParentTopic.ID != ancestorTopic.ID {
		t.Fatalf("Expected child topic 2 to be placed under the ancestor topic, got: %s", newParentTopic)
	}

	newParentTopic, err = childLink1.ParentTopics().One(mutator.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if newParentTopic.ID != ancestorTopic.ID {
		t.Fatalf("Expected child link 1 to be placed under the ancestor topic, got: %s", newParentTopic)
	}
}

func TestDeleteTopicTimeRange(t *testing.T) {
	m := newMutator(t, testViewer)

	topic, _ := m.createTopic(testViewer.Login, m.defaultRepo().Name, "A new topic")
	timerange := &models.Timerange{
		StartsAt:     time.Now(),
		PrefixFormat: string(models.TimeRangePrefixFormatNone),
	}

	if err := timerange.Insert(m.ctx, testDB, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	topic.TimerangeID = null.NewString(timerange.ID, true)
	if _, err := topic.Update(m.ctx, testDB, boil.Whitelist("timerange_id")); err != nil {
		t.Fatal(err)
	}

	payload, err := m.resolver.DeleteTopicTimeRange(m.ctx, models.DeleteTopicTimeRangeInput{
		TopicID: topic.ID,
	})
	if err != nil {
		t.Fatal(err)
	}

	if payload == nil {
		t.Fatal("Expected a payload")
	}

	timerange, _ = models.FindTimerange(m.ctx, testDB, *payload.DeletedTimeRangeID)
	if timerange != nil {
		t.Fatal("Expected time range to have been deleted")
	}
}

func TestChildTopicAndLinkVisibility(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	c := services.Connection{Exec: testDB, Actor: testViewer}

	result, err := c.CreateUser(ctx, "gnusto", "gnusto@frotz.com", "gnusto", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()
	testViewer2 := result.User

	var root *models.TopicValue
	if root, err = m.defaultRepo().RootTopic(ctx, testDB, testViewer2.DefaultView()); err != nil {
		t.Fatal(err)
	}

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Child topic")
	defer cleanup()

	m.addParentTopicToTopic(topic, root)

	_, cleanup = m.createLink(testViewer.Login, repoName, "Private link", "https://www.nytimes.com")
	defer cleanup()

	query := rootResolver.Topic()

	var root2 *models.TopicValue
	if root2, err = m.defaultRepo().RootTopic(ctx, testDB, testViewer2.DefaultView()); err != nil {
		t.Fatal(err)
	}

	var conn *models.SearchResultItemConnection
	if conn, err = query.Search(m.ctx, root2, "Child topic", nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) > 0 {
		topic := conn.Edges[0].Node.(models.TopicValue)
		t.Fatalf("Child topic should be omitted from result for second user: %#v", topic.Name)
	}

	if conn, err = query.Search(m.ctx, root2, "Private link", nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) > 0 {
		link := conn.Edges[0].Node.(models.LinkValue)
		t.Fatalf("Private link should be omitted from result for second user: %#v", link.Title)
	}
}

func TestTopicNoSynonym(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "A topic")
	defer cleanup()

	synonyms, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if len(synonyms.Values) != 1 {
		t.Fatal("Expected there to be a single synonym")
	}

	// Should never happen
	topic.Synonyms.Marshal([]models.Synonym{})
	_, err = topic.Update(ctx, testDB, boil.Whitelist("synonyms"))
	if err != nil {
		t.Fatal(err)
	}

	// Todo: check display name
}

func TestViewerCanUpdate(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "A topic")
	defer cleanup()

	query := rootResolver.Topic()

	canUpdate, err := query.ViewerCanUpdate(ctx, topic)
	if err != nil {
		t.Fatal(err)
	}

	if !canUpdate {
		t.Fatal("First viewer should be able to update the topic")
	}

	query = rootResolver.Topic()

	c := services.New(testDB, testViewer, testFetcher)

	result, err := c.CreateUser(ctx, "gnusto", "gnusto@frotz.com", "gnusto", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()
	testActor2 := result.User

	// Change out the viewer doing the query
	topic.View.ViewerID = testActor2.ID

	canUpdate, err = query.ViewerCanUpdate(ctx, topic)
	if err != nil {
		t.Fatal(err)
	}

	if canUpdate {
		t.Fatal("Second viewer should not be able to update the topic")
	}
}

func TestViewerCannotUpdateRootTopic(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "A topic")
	defer cleanup()

	query := rootResolver.Topic()

	topic.Root = true

	canUpdate, err := query.ViewerCanUpdate(ctx, topic)
	if err != nil {
		t.Fatal(err)
	}

	if canUpdate {
		t.Fatal("Viewers should not be able to update root topic")
	}
}

func TestViewerCanDeleteSynonymWhenLessThanTwoExist(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "A topic")
	defer cleanup()

	query := rootResolver.Topic()

	canDelete, err := query.ViewerCanDeleteSynonyms(ctx, topic)
	if err != nil {
		t.Fatal(err)
	}

	if canDelete {
		t.Fatal("Viewer should not be able to delete a synonym, because there is only one")
	}
}

func TestGuestViewTopic(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repo := m.defaultRepo()

	topic, cleanup := m.createTopic(testViewer.Login, repo.Name, "A topic")
	defer cleanup()

	link, cleanup := m.createLink(testViewer.Login, repo.Name, "Public topic", "https://www.nytimes.com")
	defer cleanup()

	m.addParentTopicToLink(link, topic)
	if err := topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	resolver := resolvers.New(testDB, rootResolver.Fetcher, rootResolver.RD).Topic()

	searchString := "topic"
	conn, err := resolver.Links(ctx, topic, nil, nil, nil, nil, &searchString, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatal("Expected at least one result")
	}
}

func TestUpdateSynonyms(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Backhoe")
	defer cleanup()

	synonyms, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if len(synonyms.Values) != 1 {
		t.Fatal("Expected there to be only one synonym")
	}

	input := models.UpdateSynonymsInput{
		Synonyms: []*models.SynonymInput{
			{Locale: "fr", Name: "Pelle rétrocaveuse"},
			{Locale: "en", Name: "Backhoe"},
		},
		TopicID: topic.ID,
	}

	resolver := rootResolver.Mutation()
	if _, err := resolver.UpdateSynonyms(ctx, input); err != nil {
		t.Fatal(err)
	}

	expectedSynonyms := &models.SynonymList{
		Values: []models.Synonym{
			{Locale: "fr", Name: "Pelle rétrocaveuse"},
			{Locale: "en", Name: "Backhoe"},
		},
	}

	if err = topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	actualSynonyms, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if !reflect.DeepEqual(expectedSynonyms, actualSynonyms) {
		t.Fatalf("Expected %v, got %v", expectedSynonyms, actualSynonyms)
	}
}

func TestTopicNameFromSynonyms(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Backhoe")
	defer cleanup()

	input := models.UpdateSynonymsInput{
		Synonyms: []*models.SynonymInput{
			{Locale: "fr", Name: "Pelle rétrocaveuse"},
			{Locale: "en", Name: "Excavator"},
			{Locale: "en", Name: "Backhoe"},
		},
		TopicID: topic.ID,
	}

	resolver := rootResolver.Mutation()
	if _, err := resolver.UpdateSynonyms(ctx, input); err != nil {
		t.Fatal(err)
	}

	if err := topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	query := rootResolver.Topic()
	name, err := query.DisplayName(ctx, topic, nil)
	if err != nil {
		t.Fatal(err)
	}

	if name != "Excavator" {
		t.Fatalf("Expected display name to be 'Excavator', got '%s'", name)
	}
}

func TestGuestTopicQuery(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "Agriculture")
	defer cleanup()

	linkTitle := "4b517480670"
	link, cleanup := m.createLink(testViewer.Login, m.defaultRepo().Name, linkTitle, "https://www.4b517480670.com")
	defer cleanup()

	m.addParentTopicToLink(link, topic)

	resolver := rootResolver.Topic()

	ctx := context.Background()
	rc := resolvers.NewRequestContext(resolvers.GuestViewer)
	ctx = resolvers.WithRequestContext(ctx, rc)

	conn, err := resolver.Links(ctx, topic, nil, nil, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatal("Expected at least one topic")
	}
}

func TestActivity(t *testing.T) {
	m := newMutator(t, testViewer)
	repo := m.defaultRepo()
	resolver := rootResolver.Topic()

	topic, cleanup := m.createTopic(testViewer.Login, repo.Name, "Gnusto")
	defer cleanup()

	c := services.New(testDB, testViewer, testFetcher)

	title := "New York Times"
	result, err := c.UpsertLink(m.ctx, repo, "https://www.nytimes.com", &title, []string{topic.ID})
	defer result.Cleanup()

	connection, err := resolver.Activity(m.ctx, topic, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected at least one activity line item")
	}
}

func TestActivityVisibility(t *testing.T) {
	ctx := context.Background()
	c := services.New(testDB, testViewer, testFetcher)

	result, err := c.CreateUser(
		ctx, "Frotz", "frotz@frotz.com", "frotz", "http://some-long-url",
	)

	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	user2 := result.User
	m2 := newMutator(t, user2)
	repoName := m2.defaultRepo().Name

	topic, cleanup := m2.createTopic(user2.Login, repoName, "Gnusto")
	defer cleanup()

	linkTitle := "4b517480670"
	link, cleanup := m2.createLink(user2.Login, repoName, linkTitle, "https://www.4b517480670.com")
	defer cleanup()

	m2.addParentTopicToLink(link, topic)

	resolver := rootResolver.Topic()

	m := newMutator(t, testViewer)

	var root *models.TopicValue
	if root, err = m.defaultRepo().RootTopic(ctx, testDB, testViewer.DefaultView()); err != nil {
		t.Fatal(err)
	}

	connection, err := resolver.Activity(ctx, root, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	for _, edge := range connection.Edges {
		node := edge.Node
		if strings.Contains(node.Description, linkTitle) {
			t.Fatalf("Activity feed contains a link submitted to a private repo: %v", link.URL)
		}
	}
}

func TestUpsertTopicTimeline(t *testing.T) {
	m := newMutator(t, testViewer)
	repo := m.defaultRepo()
	resolver := rootResolver.Mutation()

	topic, cleanup := m.createTopic(testViewer.Login, repo.Name, "Gnusto")
	defer cleanup()

	input := models.UpsertTopicTimeRangeInput{
		TopicID:      topic.ID,
		StartsAt:     time.Now().Format(time.RFC3339),
		PrefixFormat: models.TimeRangePrefixFormatStartYear,
	}

	payload, err := resolver.UpsertTopicTimeRange(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if payload.TimeRangeEdge == nil {
		t.Fatal("Expected a timeline edge")
	}
}
