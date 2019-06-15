package resolvers_test

import (
	"context"
	"reflect"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/boil"
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

func TestUpdateTopicPreventsOwnTopic(t *testing.T) {
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "Agriculture")
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
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "Agriculture")
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

	topicResolver := rootResolver.Topic()

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
	t.Skip("Fix test flake or delete")

	m := newMutator(t, testActor)

	var err error
	var root *models.TopicValue

	if root, err = m.defaultRepo().RootTopic(m.ctx, testDB, testActor.DefaultView()); err != nil {
		t.Fatal(err)
	}
	if root.View == nil {
		t.Fatal("Expected a view")
	}

	topic, cleanup := m.createTopic(testActor.Login, m.defaultRepo().Name, "News organizations")
	defer cleanup()
	m.addParentTopicToTopic(topic, root)

	topicResolver := rootResolver.Topic()

	var conn models.SearchResultItemConnection

	if conn, err = topicResolver.Search(m.ctx, root, root.Name, nil, nil, nil, nil); err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatalf("Expected a result, %s", testActor.Summary())
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
		t.Fatalf("Expected root topic to show up in results, %s", testActor.Summary())
	}
}

func TestParentTopicPreloading(t *testing.T) {
	r := rootResolver.Topic()
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
	s := services.New(testDB, testActor, rootResolver.Fetcher)

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
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name
	matchingString := "695be58"
	nonMatchingString := "doesn't match"

	t1, cleanup := m.createTopic(testActor.Login, repoName, "Topic 1")
	defer cleanup()

	t2, cleanup := m.createTopic(testActor.Login, repoName, matchingString)
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
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name
	matchingString := "695be58"

	t1, cleanup := m.createTopic(testActor.Login, repoName, "Topic 1")
	defer cleanup()

	t2, cleanup := m.createTopic(testActor.Login, repoName, matchingString)
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

func TestChildTopicAndLinkVisibility(t *testing.T) {
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	var err error
	var root *models.TopicValue
	if root, err = m.defaultRepo().RootTopic(ctx, testDB, testActor.DefaultView()); err != nil {
		t.Fatal(err)
	}

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Child topic")
	defer cleanup()

	m.addParentTopicToTopic(topic, root)

	_, cleanup = m.createLink(testActor.Login, repoName, "Private link", "https://www.nytimes.com")
	defer cleanup()

	query := rootResolver.Topic()

	var root2 *models.TopicValue
	if root2, err = m.defaultRepo().RootTopic(ctx, testDB, testActor2.DefaultView()); err != nil {
		t.Fatal(err)
	}

	var conn models.SearchResultItemConnection
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
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "A topic")
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
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "A topic")
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

func TestViewerCanDeleteSynonymWhenLessThanTwoExist(t *testing.T) {
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "A topic")
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
	m := newMutator(t, testActor)
	ctx := context.Background()
	repo := m.defaultRepo()

	topic, cleanup := m.createTopic(testActor.Login, repo.Name, "A topic")
	defer cleanup()

	link, cleanup := m.createLink(testActor.Login, repo.Name, "Public topic", "https://www.nytimes.com")
	defer cleanup()

	m.addParentTopicToLink(link, topic)
	if err := topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	resolver := resolvers.New(testDB, &resolvers.GuestUser, rootResolver.Fetcher, rootResolver.RD).Topic()

	searchString := "topic"
	conn, err := resolver.Links(ctx, topic, &searchString, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatal("Expected at least one result")
	}
}

func TestUpdateSynonyms(t *testing.T) {
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Backhoe")
	defer cleanup()

	synonyms, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if len(synonyms.Values) != 1 {
		t.Fatal("Expected there to be only one synonym")
	}

	input := models.UpdateSynonymsInput{
		Synonyms: []models.SynonymInput{
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
	m := newMutator(t, testActor)
	ctx := context.Background()
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Backhoe")
	defer cleanup()

	input := models.UpdateSynonymsInput{
		Synonyms: []models.SynonymInput{
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
	name, err := query.DisplayName(ctx, topic)
	if err != nil {
		t.Fatal(err)
	}

	if name != "Excavator" {
		t.Fatalf("Expected display name to be 'Excavator', got '%s'", name)
	}
}
