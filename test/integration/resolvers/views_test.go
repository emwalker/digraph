package resolvers_test

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

var (
	ge = func(expected, actual int) bool {
		return actual >= expected
	}

	eq = func(expected, actual int) bool {
		return actual == expected
	}
)

func TestQueryView(t *testing.T) {
	ctx := context.Background()
	query := rootResolver.View()

	// When the repository is in the db
	repo, err := testViewer.DefaultRepo(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	v1 := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{repo.ID}}
	connection, err := query.Topics(ctx, v1, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected at least a root topic")
	}

	// When the repo is not in the db
	fakeID := "542d7ecc-f378-11e8-8eb2-f2801f1b9fd1"
	v2 := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{fakeID}}
	connection, err = query.Topics(ctx, v2, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) > 0 {
		t.Fatal("Did not expect a result")
	}

	// When no repo id is provided
	v3 := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{}}
	connection, err = query.Topics(ctx, v3, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected a result")
	}
}

func TestSearchTopics(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login.String, repoName, "Agriculture")
	defer cleanup()

	childTopic, cleanup := m.createTopic(testViewer.Login.String, repoName, "Crop rotation")
	defer cleanup()

	m.addParentTopicToTopic(childTopic, topic)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
		Success      func(int, int) bool
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "crop rotation",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is a full match",
			SearchString: "crop rotation",
			Count:        1,
			Success:      eq,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "crop rota",
			Count:        1,
			Success:      eq,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "rotation",
			Count:        1,
			Success:      eq,
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
			Success:      eq,
		},
	}

	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}
	viewResolver := rootResolver.View()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := viewResolver.Topics(m.ctx, view, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); !td.Success(td.Count, count) {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}

func TestSearchLinks(t *testing.T) {
	m := newMutator(t, testViewer)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testViewer.Login.String, repoName, "News organizations")
	defer cleanup()

	link, cleanup := m.createLink(testViewer.Login.String, repoName, "New York Times", "https://www.nytimes.com")
	defer cleanup()

	m.addParentTopicToLink(link, topic)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
		Success      func(int, int) bool
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is a full match",
			SearchString: "New York Times",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "New Yor",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "York Times",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is no match",
			SearchString: "134ljasf",
			Count:        0,
			Success:      eq,
		},
	}

	view := &models.View{RepositoryIds: []string{m.defaultRepo().ID}}
	viewResolver := rootResolver.View()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := viewResolver.Links(m.ctx, view, &td.SearchString, nil, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); !td.Success(td.Count, count) {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}

func TestTopicVisibility(t *testing.T) {
	ctx1 := testContext()

	c := services.New(testDB, testViewer, testFetcher)

	userResult1, err := c.CreateUser(ctx1, "Gnusto", "gnusto@gnusto.com", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer userResult1.Cleanup()

	userResult2, err := c.CreateUser(ctx1, "Frotz", "frotz@frotz.com", "http://avatar/url")
	if err != nil {
		t.Fatal(err)
	}
	defer userResult2.Cleanup()

	user1 := userResult1.User
	user2 := userResult2.User

	if user1.ID == user2.ID {
		t.Fatal("Two users should have been created")
	}

	r1, err := c.CompleteRegistration(ctx1, user1, "gnusto")
	if err != nil {
		t.Fatal(err)
	}
	defer r1.Cleanup()

	r2, err := c.CompleteRegistration(ctx1, user2, "frotz")
	if err != nil {
		t.Fatal(err)
	}
	defer r2.Cleanup()

	ctx2 := context.Background()
	rc := resolvers.NewRequestContext(user2)
	ctx2 = resolvers.WithRequestContext(ctx2, rc)

	m1 := newMutator(t, user1)
	m2 := newMutator(t, user2)

	t1, cleanup := m1.createTopic(user1.Login.String, m1.defaultRepo().Name, "News organizations")
	defer cleanup()

	t2, cleanup := m2.createTopic(user2.Login.String, m2.defaultRepo().Name, "News organizations")
	defer cleanup()

	if t1.ID == t2.ID {
		t.Fatal("Topics should not be de-duped between repos")
	}

	r := rootResolver.View()
	v1 := &models.View{ViewerID: user1.ID, RepositoryIds: []string{r1.Repository.ID}}
	v2 := &models.View{ViewerID: user2.ID, RepositoryIds: []string{r2.Repository.ID}}
	var topic *models.TopicValue

	if topic, err = r.Topic(ctx1, v1, t1.ID); err != nil {
		t.Fatal(err)
	}

	if topic == nil {
		t.Fatal("User 1 unable to fetch own topic")
	}

	if topic, err = r.Topic(ctx1, v1, t2.ID); err == nil {
		t.Fatal("User 1 able to see topic in private repo of user 2")
	}

	if topic, err = r.Topic(ctx2, v2, t2.ID); err != nil {
		t.Fatal(err)
	}

	if topic == nil {
		t.Fatal("User 2 unable to fetch own topic")
	}

	if topic, err = r.Topic(ctx2, v2, t1.ID); err == nil {
		t.Fatal("User 2 able to see topic in private repo of user 1")
	}
}

func TestTopicGraph(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := context.Background()
	r := rootResolver.View()
	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}

	str, err := r.TopicGraph(ctx, view)
	if err != nil {
		t.Fatal(err)
	}

	if str == nil {
		t.Fatal("Expected a string result")
	}

	result := struct {
		Nodes []interface{} `json:"nodes"`
		Links []interface{} `json:"links"`
	}{}

	if err = json.Unmarshal([]byte(*str), &result); err != nil {
		t.Fatal(err)
	}

	if len(result.Nodes) < 1 {
		t.Fatalf("Expected at least one node: %s", *str)
	}

	if len(result.Links) < 1 {
		t.Fatalf("Expected at least one link: %s", *str)
	}
}

func TestTopicCount(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := context.Background()
	r := rootResolver.View()
	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}

	count, err := r.TopicCount(ctx, view)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatal("Expected at least one topic")
	}
}

func TestLinkCount(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := context.Background()
	r := rootResolver.View()
	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}

	count, err := r.LinkCount(ctx, view)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatal("Expected at least one link")
	}
}

func TestViewActivity(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := context.Background()
	r := rootResolver.View()
	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}

	_, cleanup := m.createLink(testViewer.Login.String, m.defaultRepo().Name, "New York Times", "https://www.nytimes.com")
	defer cleanup()

	connection, err := r.Activity(ctx, view, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected at least one activity line item")
	}
}

func TestViewActivityVisibility(t *testing.T) {
	ctx := context.Background()
	c := services.New(testDB, testViewer, testFetcher)

	result, err := c.CreateUser(ctx, "Frotz", "frotz@frotz.com", "http://avatar/url")

	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	user2 := result.User

	registrationResult, err := c.CompleteRegistration(ctx, user2, "frotz")
	if err != nil {
		t.Fatal(err)
	}
	defer registrationResult.Cleanup()

	if err = user2.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	m2 := newMutator(t, user2)

	linkTitle := "4b517480670"
	link, cleanup := m2.createLink(user2.Login.String, m2.defaultRepo().Name, linkTitle, "https://www.4b517480670.com")
	defer cleanup()

	m := newMutator(t, testViewer)
	r := rootResolver.View()
	view := &models.View{ViewerID: testViewer.ID, RepositoryIds: []string{m.defaultRepo().ID}}

	connection, err := r.Activity(ctx, view, nil, nil, nil, nil)
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

func TestReviewNeeded(t *testing.T) {
	m := newMutator(t, testViewer)
	ctx := testContext()

	linkTitle := "4b517480670"
	link, cleanup := m.createLink(testViewer.Login.String, m.defaultRepo().Name, linkTitle, "https://www.4b517480670.com")
	defer cleanup()

	count, err := link.UserLinkReviews(qm.Where("user_id = ?", testViewer.ID)).Count(ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatal("Expected a pending link review to have been created")
	}

	view := &models.View{ViewerID: testViewer.Login.String, RepositoryIds: []string{m.defaultRepo().ID}}
	resolver := rootResolver.View()

	reviewed := false
	conn, err := resolver.Links(ctx, view, nil, nil, nil, nil, nil, &reviewed)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) < 1 {
		t.Fatal("There should at least one unreviewed link")
	}

	c := services.Connection{Exec: testDB, Actor: testViewer}
	_, err = c.ReviewLink(ctx, link.Link, true)
	if err != nil {
		t.Fatal(err)
	}

	conn, err = resolver.Links(ctx, view, nil, nil, nil, nil, nil, &reviewed)
	if err != nil {
		t.Fatal(err)
	}

	if len(conn.Edges) > 0 {
		t.Fatal("There should be no unreviewed links now")
	}
}

func TestDefaultOrganization(t *testing.T) {
	ctx := testContext()
	resolver := rootResolver.View()
	view := &models.View{}

	org, err := resolver.DefaultOrganization(ctx, view)
	if err != nil {
		t.Fatal(err)
	}

	if !org.Public || org.Login != "wiki" {
		t.Fatal("Expected the public organization")
	}
}

func TestGuestViewer(t *testing.T) {
	ctx := context.Background()
	resolver := resolvers.New(rootResolver.DB, rootResolver.Fetcher, rootResolver.Redis).View()

	viewer, err := resolver.Viewer(ctx, resolvers.GuestView)
	if err != nil {
		t.Fatal(err)
	}

	if !viewer.IsGuest() {
		t.Fatalf("Expected the guest user: %v", viewer)
	}
}
