package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
	in "github.com/emwalker/digraph/test/integration"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

func TestUpsertBadLink(t *testing.T) {
	title := "topic name"
	service := services.UpsertLink{
		Actor:         in.Actor,
		Repository:    in.Repository,
		ProvidedTitle: &title,
	}

	result, err := service.Call(context.Background(), in.DB)
	in.Must(err)

	if len(result.Alerts) < 1 {
		t.Fatal("Expected one or more alerts")
	}

	if result.Link != nil {
		t.Fatal("A link should not have been created")
	}
}

func TestLinkHasATopic(t *testing.T) {
	ctx := context.Background()
	in.NewMutator(in.MutatorOptions{}).DeleteLinksByURL("http://some.url.com/")

	title := "A title"
	service := services.UpsertLink{
		Actor:         in.Actor,
		Repository:    in.Repository,
		ProvidedTitle: &title,
		ProvidedURL:   "http://some.url.com/",
	}

	result, err := service.Call(ctx, in.DB)
	in.Must(err)

	if !result.LinkCreated {
		t.Fatal("Expected link to be a new one")
	}

	if result.Link.R != nil {
		t.Fatal("There should be no preloads on the link")
	}

	topics, err := result.Link.ParentTopics().All(ctx, in.DB)
	in.Must(err)

	if len(topics) < 1 {
		t.Fatal("Expected the link to be added to the root topic")
	}
}

func TestUpsertExistingLinkWithTopic(t *testing.T) {
	// https://github.com/emwalker/digraph/issues/13
	in.NewMutator(in.MutatorOptions{}).DeleteLinksByURL("http://some.url.com/")
	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()

	topicResult, err := c.UpsertTopic(ctx, in.Repository, "62ce187241e", nil, []string{})
	in.Must(err)

	// Initial creation
	title := "A title"
	service := services.UpsertLink{
		Actor:          in.Actor,
		Repository:     in.Repository,
		ProvidedTitle:  &title,
		ProvidedURL:    "http://some.url.com/",
		ParentTopicIds: []string{topicResult.Topic.ID},
	}

	linkResult, err := service.Call(ctx, in.DB)
	in.Must(err)

	if !linkResult.LinkCreated {
		t.Fatal("Expected link to be a new one")
	}

	service = services.UpsertLink{
		Actor:          in.Actor,
		Repository:     in.Repository,
		ProvidedTitle:  &title,
		ProvidedURL:    "http://some.url.com/",
		ParentTopicIds: []string{topicResult.Topic.ID},
	}

	// A second upsert
	linkResult, err = service.Call(ctx, in.DB)
	in.Must(err)

	topics, err := linkResult.Link.ParentTopics().All(ctx, in.DB)
	in.Must(err)

	for _, topic := range topics {
		if topic.Root {
			t.Fatal("The root topic should not be automatically added to a link that already has a topic")
		}
	}
}

func TestUserLinkHistory(t *testing.T) {
	in.NewMutator(in.MutatorOptions{}).DeleteTopicsByName("62ce1872411")

	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, in.Repository, "62ce1872411", nil, []string{})
	in.Must(err)

	topic := result.Topic
	actor := in.Actor

	prevCount, _ := actor.UserLinks().Count(ctx, in.DB)
	var nextCount int64

	// A log is added for an upsert
	title := "A title"
	service := services.UpsertLink{
		Actor:          in.Actor,
		Repository:     in.Repository,
		ProvidedTitle:  &title,
		ProvidedURL:    "http://frotz.com/",
		ParentTopicIds: []string{topic.ID},
	}

	upsertResult, err := service.Call(ctx, in.DB)
	in.Must(err)

	nextCount, _ = actor.UserLinks().Count(ctx, in.DB)
	if (prevCount + 1) != nextCount {
		t.Fatal("Expected a new user link record to be created for the upsert")
	}

	userLink, err := actor.UserLinks(qm.OrderBy("created_at desc")).One(ctx, in.DB)
	in.Must(err)

	linkTopicCount, err := userLink.UserLinkTopics().Count(ctx, in.DB)
	in.Must(err)

	if linkTopicCount < 1 {
		t.Fatal("Expected at least one row to be added to user_link_topics")
	}

	// A log is not added for a delete at this time
	_, err = c.DeleteLink(ctx, in.Repository, upsertResult.Link)
	in.Must(err)
}

func TestUserLinkReviewAdded(t *testing.T) {
	ctx := context.Background()
	actor := in.Actor

	in.NewMutator(in.MutatorOptions{}).DeleteLinksByURL("http://frotz.com/")
	_, err := actor.UserLinkReviews().DeleteAll(ctx, in.DB)
	in.Must(err)

	prevCount, err := actor.UserLinkReviews().Count(ctx, in.DB)
	in.Must(err)

	title := "A title"
	service := services.UpsertLink{
		Actor:         in.Actor,
		Repository:    in.Repository,
		ProvidedTitle: &title,
		ProvidedURL:   "http://frotz.com/",
	}
	_, err = service.Call(ctx, in.DB)
	in.Must(err)

	nextCount, err := actor.UserLinkReviews().Count(ctx, in.DB)
	in.Must(err)

	if prevCount+1 != nextCount {
		t.Fatalf("Expected a user-link-review record to be created")
	}
}

func TestReviewLink(t *testing.T) {
	ctx := context.Background()
	actor := in.Actor
	c := services.Connection{Exec: in.DB, Actor: actor}

	title := "A title"
	service := services.UpsertLink{
		Actor:         in.Actor,
		Repository:    in.Repository,
		ProvidedTitle: &title,
		ProvidedURL:   "http://frotz.com/",
	}
	upsertResult, err := service.Call(ctx, in.DB)
	in.Must(err)

	link := upsertResult.Link

	reviews, err := actor.UserLinkReviews(qm.Where("link_id = ?", link.ID)).All(ctx, in.DB)
	in.Must(err)

	if len(reviews) != 1 {
		t.Fatal("Expected there to be a single user-link-review")
	}

	review := reviews[0]
	if !review.ReviewedAt.IsZero() {
		t.Fatal("Expected the review to be pending")
	}

	result, err := c.ReviewLink(ctx, link, true)
	if result.Review.ReviewedAt.IsZero() {
		t.Fatal("Expected the review to be pending")
	}
}
