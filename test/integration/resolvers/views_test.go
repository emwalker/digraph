package resolvers_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/emwalker/digraph/services"
	helpers "github.com/emwalker/digraph/testing"
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
	query := (&resolvers.Resolver{DB: testDB}).View()

	// When the repository is in the db
	repo, err := testActor.DefaultRepo(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	v1 := &models.View{ViewerID: testActor.ID, RepositoryIds: []string{repo.ID}}
	connection, err := query.Topics(ctx, v1, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected a result")
	}

	// When the repo is not in the db
	fakeId := "542d7ecc-f378-11e8-8eb2-f2801f1b9fd1"
	v2 := &models.View{ViewerID: testActor.ID, RepositoryIds: []string{fakeId}}
	connection, err = query.Topics(ctx, v2, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) > 0 {
		t.Fatal("Did not expect a result")
	}

	// When no repo id is provided
	v3 := &models.View{ViewerID: testActor.ID, RepositoryIds: []string{}}
	connection, err = query.Topics(ctx, v3, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if len(connection.Edges) < 1 {
		t.Fatal("Expected a result")
	}
}

func TestSearchTopics(t *testing.T) {
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	childTopic, cleanup := m.createTopic("Crop rotation")
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

	view := &models.View{ViewerID: testActor.ID, RepositoryIds: []string{m.defaultRepo().ID}}
	viewResolver := (&resolvers.Resolver{DB: testDB}).View()

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
	m := newMutator(t, testActor)

	topic, cleanup := m.createTopic("News organizations")
	defer cleanup()

	link, cleanup := m.createLink("New York Times", "https://www.nytimes.com")
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
			SearchString: "astronomy",
			Count:        0,
			Success:      eq,
		},
	}

	view := &models.View{RepositoryIds: []string{m.defaultRepo().ID}}
	viewResolver := (&resolvers.Resolver{DB: testDB}).View()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := viewResolver.Links(m.ctx, view, &td.SearchString, nil, nil, nil, nil)
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
	ctx := context.Background()
	c := services.Connection{Exec: testDB, Actor: testActor}

	r1, cleanup, err := helpers.CreateUser(
		c,
		ctx,
		"Gnusto",
		"gnusto@gnusto.com",
		"gnusto",
		"http://some-long-url",
	)
	if err != nil {
		t.Fatal(err)
	}
	defer cleanup()

	r2, cleanup, err := helpers.CreateUser(
		c,
		ctx,
		"Frotz",
		"frotz@frotz.com",
		"frotz",
		"http://some-long-url",
	)
	if err != nil {
		t.Fatal(err)
	}
	defer cleanup()

	if r1.User.ID == r2.User.ID {
		t.Fatal("Two users should have been created")
	}

	m1 := newMutator(t, r1.User)
	m2 := newMutator(t, r2.User)

	t1, cleanup := m1.createTopic("News organizations")
	defer cleanup()

	t2, cleanup := m2.createTopic("News organizations")
	defer cleanup()

	if t1.ID == t2.ID {
		t.Fatal("Topics should not be de-duped between repos")
	}

	r := (&resolvers.Resolver{DB: testDB}).View()
	v1 := &models.View{ViewerID: r1.User.ID, RepositoryIds: []string{r1.Repository.ID}}
	v2 := &models.View{ViewerID: r2.User.ID, RepositoryIds: []string{r2.Repository.ID}}
	var topic *models.Topic

	if topic, err = r.Topic(ctx, v1, t1.ID); err != nil {
		t.Fatal(err)
	}

	if topic == nil {
		t.Fatal("User 1 unable to fetch own topic")
	}

	if topic, err = r.Topic(ctx, v1, t2.ID); err == nil {
		t.Fatal("User 1 able to see topic in private repo of user 2")
	}

	if topic, err = r.Topic(ctx, v2, t2.ID); err != nil {
		t.Fatal(err)
	}

	if topic == nil {
		t.Fatal("User 2 unable to fetch own topic")
	}

	if topic, err = r.Topic(ctx, v2, t1.ID); err == nil {
		t.Fatal("User 2 able to see topic in private repo of user 1")
	}
}

func TestFetchRepositoryFromView(t *testing.T) {
	ctx := context.Background()
	m := newMutator(t, testActor)

	repo := m.defaultRepo()
	org, err := repo.Organization().One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	view := &models.View{RepositoryIds: []string{repo.ID}}
	viewResolver := (&resolvers.Resolver{DB: testDB}).View()

	cases := []struct {
		Name     string
		RepoID   *string
		RepoName *string
		OrgLogin *string
	}{
		{
			Name:     "When the repository id is provided",
			RepoID:   &repo.ID,
			RepoName: nil,
			OrgLogin: nil,
		},
		{
			Name:     "When the org login and repo name are provied",
			RepoID:   nil,
			RepoName: &repo.Name,
			OrgLogin: &org.Login,
		},
		{
			Name:     "When only the org login is provided",
			RepoID:   nil,
			RepoName: nil,
			OrgLogin: &org.Login,
		},
	}

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			// Using a name and org login
			fetchedRepo, err := viewResolver.Repository(ctx, view, td.RepoID, td.RepoName, td.OrgLogin)
			if err != nil {
				t.Fatal(err)
			}

			if repo.ID != fetchedRepo.ID {
				t.Fatalf("Expected repo %s, got repo %s", repo.ID, fetchedRepo.ID)
			}
		})
	}
}
