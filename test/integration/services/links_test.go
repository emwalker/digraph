package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertBadLink(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}

	result, err := c.UpsertLink(context.Background(), defaultRepo, "topic name", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if len(result.Alerts) < 1 {
		t.Fatal("Expected one or more alerts")
	}

	if result.Link != nil {
		t.Fatal("A link should not have been created")
	}
}

func TestLinkHasATopic(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	title := "A title"
	result, err := c.UpsertLink(ctx, defaultRepo, "http://some.url.com/", &title, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	if !result.LinkCreated {
		t.Fatal("Expected link to be a new one")
	}

	topics, err := result.Link.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		t.Fatal(err)
	}

	if len(topics) < 1 {
		t.Fatal("Expected the link to be added to the root topic")
	}
}

func TestUpsertExistingLinkWithTopic(t *testing.T) {
	// https://github.com/emwalker/digraph/issues/13
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "62ce187241e", nil, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer topicResult.Cleanup()

	// Initial creation
	title := "A title"
	linkResult, err := c.UpsertLink(ctx, defaultRepo, "http://some.url.com/", &title, []string{topicResult.Topic.ID})
	if err != nil {
		t.Fatal(err)
	}
	defer linkResult.Cleanup()

	if !linkResult.LinkCreated {
		t.Fatal("Expected link to be a new one")
	}

	// A second upsert
	linkResult, err = c.UpsertLink(ctx, defaultRepo, "http://some.url.com/", &title, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer linkResult.Cleanup()

	topics, err := linkResult.Link.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		t.Fatal(err)
	}

	for _, topic := range topics {
		if topic.Root {
			t.Fatal("The root topic should not be automatically added to a link that already has a topic")
		}
	}
}

func TestUserLinkHistory(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "62ce1872411", nil, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer result.Cleanup()

	topic := result.Topic

	prevCount, _ := testActor.UserLinks().Count(ctx, testDB)
	var nextCount int64

	// A log is added for an upsert
	title := "A title"
	upsertResult, err := c.UpsertLink(ctx, defaultRepo, "http://frotz.com/", &title, []string{topic.ID})
	if err != nil {
		t.Fatal(err)
	}
	defer upsertResult.Cleanup()

	nextCount, _ = testActor.UserLinks().Count(ctx, testDB)
	if (prevCount + 1) != nextCount {
		t.Fatal("Expected a new user link record to be created for the upsert")
	}

	userLink, err := testActor.UserLinks(qm.OrderBy("created_at desc")).One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	linkTopicCount, err := userLink.UserLinkTopics().Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if linkTopicCount < 1 {
		t.Fatal("Expected at least one row to be added to user_link_topics")
	}

	// A log is not added for a delete at this time
	deleteResult, err := c.DeleteLink(ctx, defaultRepo, upsertResult.Link)
	if err != nil {
		t.Fatal(err)
	}
	defer deleteResult.Cleanup()
}

func TestUserLinkReviewAdded(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	prevCount, _ := testActor.UserLinkReviews().Count(ctx, testDB)

	title := "A title"
	upsertResult, err := c.UpsertLink(ctx, defaultRepo, "http://frotz.com/", &title, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer upsertResult.Cleanup()

	nextCount, _ := testActor.UserLinkReviews().Count(ctx, testDB)

	if prevCount+1 != nextCount {
		t.Fatalf("Expected a user-link-review record to be created")
	}
}

func TestReviewLink(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	title := "A title"
	upsertResult, err := c.UpsertLink(ctx, defaultRepo, "http://frotz.com/", &title, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer upsertResult.Cleanup()

	link := upsertResult.Link

	reviews, err := testActor.UserLinkReviews(qm.Where("link_id = ?", link.ID)).All(ctx, c.Exec)
	if err != nil {
		t.Fatal(err)
	}

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
