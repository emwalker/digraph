package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/services"
	in "github.com/emwalker/digraph/golang/test/integration"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

func TestUpsertLink(t *testing.T) {
	url := "https://gnusto.blog"
	m := newMutator(t, testViewer)

	_, err := models.Links(qm.Where("url = ?", url)).DeleteAll(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	topic, err := models.Topics().One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	input := models.UpsertLinkInput{
		AddParentTopicIds: []string{topic.ID},
		OrganizationLogin: testViewer.Login.String,
		RepositoryName:    m.defaultRepo().Name,
		URL:               url,
	}

	payload1, err := m.resolver.UpsertLink(m.ctx, input)
	if err != nil {
		m.t.Fatal(err)
	}

	link := payload1.LinkEdge.Node

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
}

func TestUpdateParentTopics(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login.String, repoName, "Gnusto's Blog", "https://gnusto.blog")
	defer in.Must(cleanup())

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

	_, cleanup := m.createTopic(testViewer.Login.String, m.defaultRepo().Name, "Something")
	defer in.Must(cleanup())

	link, cleanup := m.createLink(testViewer.Login.String, m.defaultRepo().Name, "Gnusto's Blog", "https://gnusto.blog")
	defer in.Must(cleanup())

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

	org, err := models.Organizations(qm.Where("login = ?", testViewer.Login.String)).One(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteRepositoriesByName("r1", "r2")

	service := services.CreateRepository{
		Organization: org,
		Name:         "r1",
		Owner:        testViewer,
	}
	r1, err := service.Call(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	service = services.CreateRepository{
		Organization: org,
		Name:         "r2",
		Owner:        testViewer,
	}
	r2, err := service.Call(m.ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	_, cleanup := m.createTopic(testViewer.Login.String, r1.Repository.Name, "Something")
	defer in.Must(cleanup())

	link, cleanup := m.createLink(testViewer.Login.String, r2.Repository.Name, "Gnusto's Blog", "https://gnusto.blog")
	defer in.Must(cleanup())

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

	link, cleanup := m.createLink(testViewer.Login.String, m.defaultRepo().Name, "Some link", "http://some.com/link")
	defer in.Must(cleanup())

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

func TestReviewLink(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login.String, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e")
	defer in.Must(cleanup())

	review, err := link.UserLinkReviews(qm.Where("user_id = ?", testViewer.ID)).One(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if review.ReviewedAt.Valid {
		t.Fatal("Expected review to be pending")
	}

	resolver := rootResolver.Mutation()
	_, err = resolver.ReviewLink(m.ctx, models.ReviewLinkInput{
		LinkID:   link.ID,
		Reviewed: true,
	})
	if err != nil {
		t.Fatal(err)
	}

	if err = review.Reload(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}

	if !review.ReviewedAt.Valid {
		t.Fatalf("Expected review to have been completed: %v", review.ReviewedAt)
	}
}

func TestViewerReview(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	link, cleanup := m.createLink(testViewer.Login.String, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e.com")
	defer in.Must(cleanup())

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

	link, cleanup := m.createLink(testViewer.Login.String, repoName, "b64c9bf1c62e", "http://b64c9bf1c62e")
	defer in.Must(cleanup())

	topic, cleanup := m.createTopic(testViewer.Login.String, repoName, "A")
	defer in.Must(cleanup())
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
