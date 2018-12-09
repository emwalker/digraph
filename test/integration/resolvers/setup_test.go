package resolvers_test

import (
	"context"
	"database/sql"
	"log"
	"os"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/emwalker/digraph/services"
	"github.com/emwalker/digraph/services/pageinfo"
	_ "github.com/lib/pq"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

const orgId = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

var (
	testDB *sql.DB
	defaultRepo *models.Repository
)

type testFetcher struct{}

type mutator struct {
	t        *testing.T
	db       *sql.DB
	ctx      context.Context
	resolver models.MutationResolver
}

func TestMain(m *testing.M) {
	services.Fetcher = &testFetcher{}
	testDB = newTestDb()
	defer testDB.Close()

	var err error

	defaultRepo, err = models.Repositories(
		qm.Where("organization_id = ? and system", orgId),
	).One(context.Background(), testDB)
	if err != nil {
		panic(err)
	}

	os.Exit(m.Run())
}

func newTestDb() *sql.DB {
	var err error
	if testDB, err = sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable"); err != nil {
		log.Fatal("Unable to connect to the database", err)
	}
	return testDB
}

func newView() *models.View {
	return &models.View{OrganizationIds: []string{orgId}}
}

func newMutator(t *testing.T) mutator {
	ctx := context.Background()
	actor, err := models.Users().One(ctx, testDB)
	if err != nil {
		panic(err)
	}

	resolver := &resolvers.MutationResolver{
		&resolvers.Resolver{DB: testDB, Actor: actor},
	}
	return mutator{t, testDB, ctx, resolver}
}

func (f *testFetcher) FetchPage(url string) (*pageinfo.PageInfo, error) {
	title := "Gnusto's blog"
	return &pageinfo.PageInfo{
		URL:   url,
		Title: &title,
	}, nil
}

func (m mutator) addParentTopicToTopic(child, parent *models.Topic) {
	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(m.ctx, testDB)
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

func (m mutator) addParentTopicToLink(link *models.Link, topic *models.Topic) {
	input := models.UpdateLinkTopicsInput{
		LinkID:         link.ID,
		ParentTopicIds: []string{topic.ID},
	}

	if _, err := m.resolver.UpdateLinkTopics(m.ctx, input); err != nil {
		m.t.Fatal(err)
	}
}

func (m mutator) deleteTopic(topic models.Topic) {
	count, err := topic.Delete(m.ctx, m.db)
	if err != nil {
		m.t.Fatal(err)
	}

	if count != int64(1) {
		m.t.Fatal("Expected a single row to be deleted")
	}
}

func (m mutator) createTopic(name string) (*models.Topic, func()) {
	parentTopic, err := models.Topics(qm.Where("name like 'Everything'")).One(m.ctx, m.db)
	if err != nil {
		m.t.Fatal(err)
	}

	input := models.UpsertTopicInput{
		Name:           name,
		RepositoryID:   defaultRepo.ID,
		TopicIds:       []string{parentTopic.ID},
	}

	payload, err := m.resolver.UpsertTopic(m.ctx, input)
	if err != nil {
		m.t.Fatal(err)
	}

	topic := payload.TopicEdge.Node

	cleanup := func() {
		m.deleteTopic(topic)
	}

	return &topic, cleanup
}

func (m mutator) createLink(title, url string) (*models.Link, func()) {
	payload1, err := m.resolver.UpsertLink(m.ctx, models.UpsertLinkInput{
		AddParentTopicIds: []string{},
		RepositoryID:      defaultRepo.ID,
		Title:             &title,
		URL:               url,
	})
	if err != nil {
		m.t.Fatal(err)
	}

	link := payload1.LinkEdge.Node

	cleanup := func() {
		count, err := link.Delete(m.ctx, testDB)
		if err != nil {
			m.t.Fatal(err)
		}

		if count != int64(1) {
			m.t.Fatal("Expected at least one updated record")
		}
	}

	return &link, cleanup
}
