package resolvers_test

import (
	"context"
	"database/sql"
	"log"
	"os"
	"testing"
	"time"

	"github.com/emwalker/digraph/golang/internal/loaders"
	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/redis"
	"github.com/emwalker/digraph/golang/internal/resolvers"
	"github.com/emwalker/digraph/golang/internal/services"
	"github.com/emwalker/digraph/golang/internal/services/pageinfo"
	_ "github.com/lib/pq"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

const orgID = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

type testFetcherT struct{}

var (
	testDB        *sql.DB
	testFetcher   *testFetcherT
	testViewer    *models.User
	rootResolver  *resolvers.Resolver
	testSessionID string
)

type mutator struct {
	actor    *models.User
	ctx      context.Context
	db       *sql.DB
	resolver models.MutationResolver
	t        *testing.T
}

func testContext() context.Context {
	rc := resolvers.NewRequestContext(testViewer)
	ctx := context.Background()
	return resolvers.WithRequestContext(ctx, rc)
}

func (m mutator) repo(login string) *models.Repository {
	repo, err := m.actor.OwnerRepositories(
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.Where("repositories.system and o.login = ?", login),
	).One(m.ctx, testDB)

	if err != nil {
		panic(err)
	}

	return repo
}

func (m mutator) defaultRepo() *models.Repository {
	return m.repo(m.actor.Login.String)
}

func TestMain(m *testing.M) {
	var err error
	testRD := redis.NewTestConnection(&redis.Options{})
	testDB = newTestDb()
	defer testDB.Close()

	testViewer, err = models.Users(
		qm.Load("SelectedRepository"),
		qm.Where("users.selected_repository_id is not null"),
	).One(context.Background(), testDB)
	if err != nil {
		panic(err)
	}

	username := testViewer.GithubUsername.Ptr()
	avatarURL := testViewer.GithubAvatarURL.Ptr()

	c := services.New(testDB, testViewer, nil)
	result, err := c.CreateGithubSession(
		context.Background(), testViewer.Name, testViewer.PrimaryEmail, *username, *avatarURL,
	)
	if err != nil {
		panic(err)
	}

	testSessionID = result.Session.ID

	rootResolver = resolvers.New(testDB, testFetcher, testRD)

	os.Exit(m.Run())
}

func newTestDb() *sql.DB {
	var err error
	if testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable"); err != nil {
		log.Fatal("Unable to connect to the database", err)
	}
	return testDB
}

func newMutator(t *testing.T, actor *models.User) mutator {
	resolver := &resolvers.MutationResolver{
		&resolvers.Resolver{DB: testDB, Fetcher: rootResolver.Fetcher},
	}

	ctx := context.Background()
	rc := resolvers.NewRequestContext(actor)
	ctx = resolvers.WithRequestContext(ctx, rc)
	ctx = loaders.AddToContext(ctx, testDB, 1*time.Millisecond)

	return mutator{
		actor:    actor,
		ctx:      ctx,
		db:       testDB,
		resolver: resolver,
		t:        t,
	}
}

func (f *testFetcherT) FetchPage(url string) (*pageinfo.PageInfo, error) {
	title := "Gnusto's blog"
	return &pageinfo.PageInfo{
		URL:   url,
		Title: &title,
	}, nil
}

func (m mutator) addParentTopicToTopic(child, parent *models.TopicValue) {
	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(context.Background(), testDB)
	if err != nil {
		m.t.Fatal(err)
	}

	input := models.UpdateTopicParentTopicsInput{
		TopicID:        child.ID,
		ParentTopicIds: []string{everything.ID, parent.ID},
	}

	if _, err := m.resolver.UpdateTopicParentTopics(m.ctx, input); err != nil {
		m.t.Fatal(err)
	}
}

func (m mutator) addParentTopicToLink(link *models.LinkValue, topic *models.TopicValue) {
	parentTopics, err := link.ParentTopics().All(m.ctx, m.db)
	if err != nil {
		m.t.Fatal(err)
	}

	topicIds := make([]string, len(parentTopics)+1)
	for i, parentTopic := range parentTopics {
		topicIds[i] = parentTopic.ID
	}
	topicIds[len(parentTopics)] = topic.ID

	input := models.UpdateLinkTopicsInput{
		LinkID:         link.ID,
		ParentTopicIds: topicIds,
	}

	if _, err := m.resolver.UpdateLinkTopics(m.ctx, input); err != nil {
		m.t.Fatal(err)
	}
}

func (m mutator) deleteTopic(topic *models.TopicValue) {
	count, err := topic.Delete(m.ctx, m.db)
	if err != nil {
		m.t.Fatal(err)
	}

	if count != int64(1) {
		m.t.Fatal("Expected a single row to be deleted")
	}
}

func (m mutator) createTopic(orgLogin, repoName, name string) (*models.TopicValue, services.CleanupFunc) {
	parentTopic, err := models.Topics(qm.Where("name like 'Everything'")).One(m.ctx, m.db)
	if err != nil {
		m.t.Fatal(err)
	}

	input := models.UpsertTopicInput{
		Name:              name,
		OrganizationLogin: orgLogin,
		RepositoryName:    repoName,
		TopicIds:          []string{parentTopic.ID},
	}

	payload, err := m.resolver.UpsertTopic(m.ctx, input)
	if err != nil {
		m.t.Fatal(err)
	}

	topic := payload.TopicEdge.Node

	cleanup := func() error {
		m.deleteTopic(topic)
		return nil
	}

	return topic, cleanup
}

func (m mutator) createLink(orgLogin, repoName, title, url string) (*models.LinkValue, services.CleanupFunc) {
	payload1, err := m.resolver.UpsertLink(m.ctx, models.UpsertLinkInput{
		AddParentTopicIds: []string{},
		OrganizationLogin: orgLogin,
		RepositoryName:    repoName,
		Title:             &title,
		URL:               url,
	})

	if err != nil {
		m.t.Fatal(err)
	}

	link := payload1.LinkEdge.Node

	cleanup := func() error {
		_, err := models.UserLinks(qm.Where("link_id = ?", link.ID)).DeleteAll(m.ctx, testDB)
		if err != nil {
			m.t.Fatal(err)
		}

		count, err := link.Delete(m.ctx, testDB)
		if err != nil {
			m.t.Fatal(err)
		}

		if count != int64(1) {
			log.Printf("Expected at least one updated record, but none was updated")
		}
		return nil
	}

	return link, cleanup
}
