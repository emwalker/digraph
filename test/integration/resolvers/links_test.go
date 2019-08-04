package resolvers_test

import (
	"strings"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertLink(t *testing.T) {
	m := newMutator(t, testViewer)

	topic, err := models.Topics().One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	input := models.UpsertLinkInput{
		AddParentTopicIds: []string{topic.ID},
		OrganizationLogin: testViewer.Login,
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
		_, err = models.UserLinks(
			qm.Where("link_id = ? and user_id = ?", link.ID, testViewer.ID),
		).DeleteAll(m.ctx, m.db)
		if err != nil {
			t.Fatal(err)
		}

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
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login, repoName, "Gnusto's Blog", "https://gnusto.blog")
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

func TestAvailableTopicsForLinks(t *testing.T) {
	m := newMutator(t, testViewer)

	_, cleanup := m.createTopic(testViewer.Login, m.defaultRepo().Name, "Something")
	defer cleanup()

	link, cleanup := m.createLink(testViewer.Login, m.defaultRepo().Name, "Gnusto's Blog", "https://gnusto.blog")
	defer cleanup()

	query := rootResolver.Link()

	conn, err := query.AvailableParentTopics(m.ctx, link, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 2 {
		t.Fatal("Expected at least one topic edge")
	}
}

func TestAvailableTopicsForLinksFromOtherRepos(t *testing.T) {
	m := newMutator(t, testViewer)
	s := services.New(testDB, testViewer, rootResolver.Fetcher)

	org, err := models.Organizations(qm.Where("login = ?", testViewer.Login)).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	r1, err := s.CreateRepository(m.ctx, org, "r1", testViewer, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r1.Cleanup()

	r2, err := s.CreateRepository(m.ctx, org, "r2", testViewer, false)
	if err != nil {
		t.Fatal(err)
	}
	defer r2.Cleanup()

	_, cleanup := m.createTopic(testViewer.Login, r1.Repository.Name, "Something")
	defer cleanup()

	link, cleanup := m.createLink(testViewer.Login, r2.Repository.Name, "Gnusto's Blog", "https://gnusto.blog")
	defer cleanup()

	query := rootResolver.Link()

	conn, err := query.AvailableParentTopics(m.ctx, link, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 2 {
		t.Fatal("Expected at least one topic edge")
	}
}

func TestDeleteLink(t *testing.T) {
	m := newMutator(t, testViewer)

	link, cleanup := m.createLink(testViewer.Login, m.defaultRepo().Name, "Some link", "http://some.com/link")
	defer cleanup()

	payload, err := m.resolver.DeleteLink(m.ctx, models.DeleteLinkInput{LinkID: link.ID})
	if err != nil {
		t.Fatal(err)
	}

	if payload == nil {
		t.Fatal("Expected a payload")
	}

	count, err := models.Links(qm.Where("id = ?", link.ID)).Count(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatal("Failed to delete link")
	}
}

func TestParentTopicsDefaultOrdering(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login, m.defaultRepo().Name, "b64c9bf1c62c", "http://b64c9bf1c62c.com")
	defer cleanup()

	tC, cleanup := m.createTopic(testViewer.Login, repoName, "C")
	defer cleanup()
	m.addParentTopicToLink(link, tC)

	tA, cleanup := m.createTopic(testViewer.Login, repoName, "A")
	defer cleanup()
	m.addParentTopicToLink(link, tA)

	tB, cleanup := m.createTopic(testViewer.Login, repoName, "B")
	defer cleanup()
	m.addParentTopicToLink(link, tB)

	if err := link.Reload(m.ctx, testDB); err != nil {
		t.Fatal(err)
	}

	query := rootResolver.Link()
	topicConnection, err := query.ParentTopics(m.ctx, link, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(topicConnection.Edges) != 4 {
		t.Fatalf("Expected 4 parent topics, got %d", len(topicConnection.Edges))
	}

	prevEdge := topicConnection.Edges[0]

	for _, edge := range topicConnection.Edges[1:] {
		if strings.Compare(prevEdge.Node.Name, edge.Node.Name) != -1 {
			t.Fatalf("Expected topics to be sorted by name: %s <=> %s", prevEdge.Node.Name, edge.Node.Name)
		}
		prevEdge = edge
	}
}

func TestReviewLink(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e")
	defer cleanup()

	review, err := link.UserLinkReviews(qm.Where("user_id = ?", testViewer.ID)).One(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if !review.ReviewedAt.IsZero() {
		t.Fatal("Expected review to be pending")
	}

	resolver := rootResolver.Mutation()
	_, err = resolver.ReviewLink(m.ctx, models.ReviewLinkInput{
		LinkID:   link.ID,
		Reviewed: true,
	})

	if err = review.Reload(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if review.ReviewedAt.IsZero() {
		t.Fatalf("Expected review to have been completed: %v", review.ReviewedAt)
	}
}

func TestViewerReview(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e.com")
	defer cleanup()

	if err := link.Reload(m.ctx, testDB); err != nil {
		t.Fatal(err)
	}

	resolver := rootResolver.Link()

	review, err := resolver.ViewerReview(m.ctx, link)
	if err != nil {
		t.Fatal(err)
	}

	if review == nil {
		t.Fatal("Expected a review to have been created")
	}

	if review.ReviewedAt != nil {
		t.Fatal("Expected a nil reviewedAt")
	}
}

func TestTotalCount(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e")
	defer cleanup()

	topic, cleanup := m.createTopic(testViewer.Login, repoName, "A")
	defer cleanup()
	m.addParentTopicToLink(link, topic)

	query := rootResolver.Topic()

	first := 100
	connection, err := query.Links(m.ctx, topic, &first, nil, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if connection.TotalCount != 1 {
		t.Fatalf("Expected a total count of 1, got %d", connection.TotalCount)
	}
}
