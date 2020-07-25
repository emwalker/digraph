package integration

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	_ "github.com/lib/pq"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

type Operator string

// Helper constants for making comparisons
const (
	Exactly     = Operator("Exactly")
	GreaterThan = Operator("GreaterThan")
	Anything    = Operator("Anything")
)

type TestFetcherT struct{}

type Mutator struct {
	Actor   *models.User
	View    *models.View
	DB      *sql.DB
	Fetcher *TestFetcherT
}

type MutatorOptions struct{}

type UpsertTopicOptions struct {
	ParentTopicIds []string
	Name           string
}

type UpsertLinkOptions struct {
	ParentTopicIds []string
	Title          string
	URL            string
}

type UpdateLinkTopicsOptions struct {
	Link           *models.LinkValue
	ParentTopicIds []string
}

// Pre-loaded objects for common tasks
var (
	DB         *sql.DB
	Fetcher    *TestFetcherT
	Actor      *models.User
	View       *models.View
	Repository *models.Repository
	Everything *models.TopicValue
)

type Condition struct {
	Operator Operator
	Expected int
}

func (c Condition) Evaluate(actual int) bool {
	switch c.Operator {
	case Exactly:
		return c.Expected == actual
	case GreaterThan:
		return actual > c.Expected
	case Anything:
		return true
	case "":
		return true
	default:
		panic(fmt.Sprintf("Do not know how to handle operator: %v", c.Operator))
	}
}

func (c Condition) Describe(actual int) string {
	switch c.Operator {
	case Exactly:
		return fmt.Sprintf("Expected %d, got %d", c.Expected, actual)
	case GreaterThan:
		return fmt.Sprintf("Expected greater than %d, got %d", c.Expected, actual)
	case Anything:
		return "Expected anything"
	case "":
		return "Expected nothing"
	default:
		panic(fmt.Sprintf("Do not know how to handle operator: %v", c))
	}
}

func (f *TestFetcherT) FetchPage(url string) (*pageinfo.PageInfo, error) {
	title := "Gnusto's blog"
	return &pageinfo.PageInfo{
		URL:   url,
		Title: &title,
	}, nil
}

// Must panics if the error is not nil
func Must(err error) {
	if err != nil {
		panic(fmt.Sprintf("There was a problem setting things up: %s", err))
	}
}

func init() {
	var err error
	ctx := context.Background()

	DB, err = NewTestDB()
	Must(err)

	Actor, err = models.Users(
		qm.Load("SelectedRepository"),
		qm.Where("users.selected_repository_id is not null"),
	).One(context.Background(), DB)
	Must(err)

	View = &models.View{ViewerID: Actor.ID}

	Repository, err = models.Repositories(
		qm.Where("name like ?", "General collection"),
	).One(ctx, DB)
	Must(err)

	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(ctx, DB)
	Must(err)
	Everything = &models.TopicValue{everything, true, View}

	Fetcher = &TestFetcherT{}
}

func NewMutator(options MutatorOptions) *Mutator {
	return &Mutator{Actor, View, DB, Fetcher}
}

func NewTestDB() (*sql.DB, error) {
	return sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
}

func (m Mutator) UpsertTopic(options UpsertTopicOptions) *models.TopicValue {
	ctx := context.Background()
	conn := services.Connection{Exec: m.DB, Actor: m.Actor}
	result, err := conn.UpsertTopic(ctx, Repository, options.Name, nil, options.ParentTopicIds)
	Must(err)
	return &models.TopicValue{result.Topic, true, View}
}

func (m Mutator) UpsertLink(options UpsertLinkOptions) *models.LinkValue {
	service := services.UpsertLink{
		Actor:          m.Actor,
		Fetcher:        m.Fetcher,
		Repository:     Repository,
		ProvidedURL:    options.URL,
		ProvidedTitle:  &options.Title,
		ParentTopicIds: options.ParentTopicIds,
	}
	result, err := service.Call(context.Background(), m.DB)
	Must(err)
	return &models.LinkValue{result.Link, true, View}
}

func (m Mutator) UpdateLinkTopics(options UpdateLinkTopicsOptions) {
	service := services.UpdateLinkTopics{
		Actor:          m.Actor,
		Link:           options.Link,
		ParentTopicIds: options.ParentTopicIds,
	}
	_, err := service.Call(context.Background(), m.DB)
	Must(err)
}

type UpdateTopicParentTopicsOptions struct {
	Topic          *models.TopicValue
	ParentTopicIds []string
}

func (m Mutator) UpdateTopicParentTopics(options UpdateTopicParentTopicsOptions) {
	service := services.UpdateTopicParentTopics{
		Actor:          m.Actor,
		Topic:          options.Topic,
		ParentTopicIds: options.ParentTopicIds,
	}
	_, err := service.Call(context.Background(), m.DB)
	Must(err)
}

func (m Mutator) DeleteTopicsByName(names ...string) {
	for _, name := range names {
		_, err := models.Topics(qm.Where("name like ?", name)).DeleteAll(context.Background(), m.DB)
		Must(err)
	}
}

func (m Mutator) DeleteLinksByURL(urls ...string) {
	for _, url := range urls {
		_, err := models.Links(qm.Where("url = ?", url)).DeleteAll(context.Background(), m.DB)
		Must(err)
	}
}
