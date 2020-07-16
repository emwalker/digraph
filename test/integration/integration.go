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

type TopicOptions struct {
	ParentTopicIds []string
	Name           string
}

type LinkOptions struct {
	ParentTopicIds []string
	Title          string
	URL            string
}

// Pre-loaded objects for common tasks
var (
	TestDB         *sql.DB
	TestFetcher    *TestFetcherT
	TestViewer     *models.User
	TestView       *models.View
	TestRepository *models.Repository
	Everything     *models.TopicValue
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

	TestDB, err = NewTestDB()
	Must(err)

	TestViewer, err = models.Users(
		qm.Load("SelectedRepository"),
		qm.Where("users.selected_repository_id is not null"),
	).One(context.Background(), TestDB)
	Must(err)

	TestView = &models.View{ViewerID: TestViewer.ID}

	TestRepository, err = models.Repositories(
		qm.Where("name like ?", "General collection"),
	).One(ctx, TestDB)
	Must(err)

	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(ctx, TestDB)
	Must(err)
	Everything = &models.TopicValue{everything, true, TestView}

	TestFetcher = &TestFetcherT{}
}

func NewMutator(options MutatorOptions) *Mutator {
	return &Mutator{TestViewer, TestView, TestDB, TestFetcher}
}

func NewTestDB() (*sql.DB, error) {
	return sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
}

func (m Mutator) MakeTopic(options TopicOptions) *models.TopicValue {
	ctx := context.Background()
	c := services.Connection{Exec: m.DB, Actor: m.Actor}
	result, err := c.UpsertTopic(ctx, TestRepository, options.Name, nil, options.ParentTopicIds)
	Must(err)
	return &models.TopicValue{result.Topic, true, TestView}
}

func (m Mutator) MakeLink(options LinkOptions) *models.LinkValue {
	ctx := context.Background()
	c := services.Connection{Exec: m.DB, Actor: m.Actor, Fetcher: m.Fetcher}
	result, err := c.UpsertLink(ctx, TestRepository, options.URL, &options.Title, options.ParentTopicIds)
	Must(err)
	return &models.LinkValue{result.Link, true, TestView}
}
