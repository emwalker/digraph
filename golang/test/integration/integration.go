package integration

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/services"
	"github.com/emwalker/digraph/golang/internal/services/pageinfo"
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
	DB      *sql.DB
	Fetcher *TestFetcherT
	View    *models.View
}

type MutatorOptions struct{}

// Pre-loaded objects for common tasks
var (
	Actor        *models.User
	DB           *sql.DB
	Everything   *models.TopicValue
	Fetcher      *TestFetcherT
	Organization *models.Organization
	Repository   *models.Repository
	View         *models.View
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
		qm.Where("id = ?", "461c87c8-fb8f-11e8-9cbc-afde6c54d881"),
	).One(context.Background(), DB)
	Must(err)

	View = &models.View{ViewerID: Actor.ID}

	Repository, err = models.Repositories(
		qm.Where("name like ?", "General collection"),
	).One(ctx, DB)
	Must(err)

	Organization, err = Repository.Organization().One(ctx, DB)
	Must(err)

	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(ctx, DB)
	Must(err)
	Everything = &models.TopicValue{everything, true, View}

	Fetcher = &TestFetcherT{}
}

func NewMutator(options MutatorOptions) *Mutator {
	return &Mutator{
		Actor:   Actor,
		DB:      DB,
		Fetcher: Fetcher,
		View:    View,
	}
}

func NewTestDB() (*sql.DB, error) {
	return sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
}

type UpsertTopicOptions struct {
	Name           string
	ParentTopicIds []string
	Repository     *models.Repository
}

func (m Mutator) UpsertTopic(options UpsertTopicOptions) *models.TopicValue {
	ctx := context.Background()
	conn := services.Connection{Exec: m.DB, Actor: m.Actor}

	repo := options.Repository
	if repo == nil {
		repo = Repository
	}

	result, err := conn.UpsertTopic(ctx, repo, options.Name, nil, options.ParentTopicIds)
	Must(err)
	return &models.TopicValue{result.Topic, true, View}
}

type UpsertLinkOptions struct {
	ParentTopicIds []string
	Repository     *models.Repository
	Title          string
	URL            string
}

func (m Mutator) UpsertLink(options UpsertLinkOptions) *models.LinkValue {
	repo := options.Repository
	if repo == nil {
		repo = Repository
	}

	service := services.UpsertLink{
		Actor:          m.Actor,
		Fetcher:        m.Fetcher,
		Repository:     repo,
		ProvidedURL:    options.URL,
		ProvidedTitle:  &options.Title,
		ParentTopicIds: options.ParentTopicIds,
	}
	result, err := service.Call(context.Background(), m.DB)
	Must(err)
	return &models.LinkValue{result.Link, true, View}
}

type UpdateLinkTopicsOptions struct {
	Link           *models.LinkValue
	ParentTopicIds []string
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

type CreateRepositoryOptions struct {
	Name         string
	Organization *models.Organization
	Owner        *models.User
	System       bool
}

func (m Mutator) CreateRepository(options CreateRepositoryOptions) *models.Repository {
	org := options.Organization
	if org == nil {
		org = Organization
	}

	owner := options.Owner
	if owner == nil {
		owner = Actor
	}

	service := services.CreateRepository{
		Organization: org,
		Name:         options.Name,
		Owner:        owner,
		System:       options.System,
	}
	repo, err := service.Call(context.Background(), m.DB)
	Must(err)

	return repo.Repository
}

func (m Mutator) DeleteRepositoriesByName(names ...string) {
	for _, name := range names {
		_, err := models.Repositories(qm.Where("name = ?", name)).DeleteAll(context.Background(), m.DB)
		Must(err)
	}
}

func (m Mutator) DeleteOrganizationsByLogin(logins ...string) {
	for _, login := range logins {
		_, err := models.Organizations(qm.Where("login = ?", login)).DeleteAll(context.Background(), m.DB)
		Must(err)
	}
}

func (m Mutator) DeleteUsersByEmail(emails ...string) {
	for _, email := range emails {
		_, err := models.Users(qm.Where("primary_email = ?", email)).DeleteAll(context.Background(), m.DB)
		Must(err)
	}
}

type CreateUserOptions struct {
	Name  string
	Email string
	Login string
}

func (m Mutator) CreateUser(options CreateUserOptions) (*models.User, *services.CompleteRegistrationResult) {
	createUser := services.CreateUser{
		Name:      options.Name,
		Email:     options.Email,
		AvatarURL: "https://example.com/avatar-url",
	}
	createUserResult, err := createUser.Call(context.Background(), m.DB)
	Must(err)

	user := createUserResult.User
	user.Login.Valid = true

	completeRegistration := services.CompleteRegistration{User: user, Login: options.Login}
	completeRegistrationResult, err := completeRegistration.Call(context.Background(), m.DB)
	Must(err)
	return user, completeRegistrationResult
}
