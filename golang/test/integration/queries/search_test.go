package queries_test

import (
	"context"
	"fmt"
	"testing"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/queries"
	in "github.com/emwalker/digraph/golang/test/integration"
)

func condition(op in.Operator, expected int) in.Condition {
	return in.Condition{Operator: op, Expected: expected}
}

func TestSearchInTopic(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteTopicsByName(
		"Child topic 1",
		"Child topic 2",
		"Child topic 3",
		"Child topic",
		"Grandchild topic 1abc",
		"Grandchild topic 2abc",
	)
	mutator.DeleteLinksByURL(
		"http://nytimely.com",
		"http://link-with-two-parents.com",
		"http://great-grandchild-1.org",
		"http://great-grandchild-2.org",
		"http://great-grandchild-3.org",
	)

	childTopic1 := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Child topic 1",
	})

	childTopic2 := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Child topic 2",
	})

	childTopic3 := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Child topic 3",
	})

	grandchildLink := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{childTopic1.ID, childTopic2.ID},
		Title:          "New York Timely",
		URL:            "http://nytimely.com",
	})

	mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{childTopic1.ID},
		Title:          "Link with two parents",
		URL:            "http://link-with-two-parents.com",
	})

	grandchildTopic1 := mutator.UpsertTopic(in.UpsertTopicOptions{
		Name:           "Grandchild topic 1abc",
		ParentTopicIds: []string{childTopic1.ID, childTopic3.ID},
	})

	grandchildTopic2 := mutator.UpsertTopic(in.UpsertTopicOptions{
		Name:           "Grandchild topic 2abc",
		ParentTopicIds: []string{childTopic2.ID},
	})

	greatGrandchildLink1 := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{grandchildTopic1.ID},
		Title:          "Great granchild link 1",
		URL:            "http://great-grandchild-1.org",
	})

	greatGrandchildLink2 := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{grandchildTopic1.ID, grandchildTopic2.ID},
		Title:          "Great granchild link 2",
		URL:            "http://great-grandchild-2.org",
	})

	greatGrandchildLink3 := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{grandchildTopic1.ID, grandchildTopic2.ID},
		Title:          "Great granchild link 3",
		URL:            "http://great-grandchild-3.org",
	})

	// Remove the topics on greatGrandchildLink3
	mutator.UpdateLinkTopics(in.UpdateLinkTopicsOptions{
		Link:           greatGrandchildLink3,
		ParentTopicIds: []string{in.Everything.ID},
	})

	// Remove childTopic3 from grandchildTopic1
	mutator.UpdateTopicParentTopics(in.UpdateTopicParentTopicsOptions{
		Topic:          grandchildTopic1,
		ParentTopicIds: []string{childTopic1.ID},
	})

	testCases := []struct {
		name         string
		searchString string
		parentTopic  *models.TopicValue
		topics       in.Condition
		links        in.Condition
	}{
		{
			name:         "When an empty string is provided",
			searchString: "",
			parentTopic:  in.Everything,
			topics:       condition(in.GreaterThan, 5),
			links:        condition(in.GreaterThan, 5),
		},
		{
			name:         "When a link matches",
			searchString: "New York Timely",
			parentTopic:  childTopic1,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 1),
		},
		{
			name:         "When a child topic matches",
			searchString: "Child topic",
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 3),
			links:        condition(in.Anything, 0),
		},
		{
			name:         "When a descendant topic matches",
			searchString: "Grandchild topic 1abc",
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 1),
			links:        condition(in.Exactly, 0),
		},
		{
			name:         "When the search is for a URL and the parent topic is not the root",
			searchString: grandchildLink.URL,
			parentTopic:  childTopic1,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 1),
		},
		{
			name:         "When the child topic is inlined in the query",
			searchString: fmt.Sprintf("in:/wiki/topics/%s %s", childTopic1.ID, grandchildLink.URL),
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 1),
		},
		{
			name:         "When there is no such url",
			searchString: fmt.Sprintf("in:/wiki/topics/%s http://no-such-url", childTopic1.ID),
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 0),
		},
		{
			name:         "When the url is a descendant url",
			searchString: fmt.Sprintf("in:/wiki/topics/%s %s", childTopic1.ID, greatGrandchildLink1.URL),
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 1),
		},
		{
			name:         "When the child topic is inlined and a link title matches",
			searchString: fmt.Sprintf("in:/wiki/topics/%s New York Timely", childTopic1.ID),
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 1),
		},
		{
			name: "When two topics are inlined",
			searchString: fmt.Sprintf(
				"in:/wiki/topics/%s in:/wiki/topics/%s",
				childTopic1.ID,
				childTopic2.ID,
			),
			parentTopic: in.Everything,
			topics:      condition(in.Exactly, 0),
			// http://nytimely.com and http://great-grandchild-2.org
			links: condition(in.Exactly, 2),
		},
		{
			name: "Descendant links from the intersection of the transitive closures are included",
			searchString: fmt.Sprintf(
				"in:/wiki/topics/%s in:/wiki/topics/%s %s",
				childTopic1.ID,
				childTopic2.ID,
				greatGrandchildLink2.URL,
			),
			parentTopic: in.Everything,
			topics:      condition(in.Exactly, 0),
			// http://great-grandchild-2.org
			links: condition(in.Exactly, 1),
		},
		{
			name:         "A topic appears in its own down set",
			searchString: fmt.Sprintf("in:/wiki/topics/%s", grandchildTopic1.ID),
			parentTopic:  in.Everything,
			topics:       condition(in.Exactly, 1),
			links:        condition(in.Anything, 2),
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			query := queries.NewSearch(td.parentTopic, &td.searchString)

			topics, err := query.DescendantTopics(ctx, mutator.DB, 100)
			in.Must(err)

			if !td.topics.Evaluate(len(topics)) {
				t.Fatalf(td.topics.Describe(len(topics)))
			}

			links, err := query.DescendantLinks(ctx, mutator.DB, 100)
			in.Must(err)

			if !td.links.Evaluate(len(links)) {
				t.Fatalf(td.links.Describe(len(links)))
			}
		})
	}
}

func TestSearchBugFixes(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})

	testCases := []struct {
		name         string
		searchString string
		topics       in.Condition
		links        in.Condition
		topicLimit   int
		linkLimit    int
	}{
		{
			name:         "When there is a URL pararameter",
			searchString: "https://some-url?parameter",
			topicLimit:   1,
			linkLimit:    1,
		},
		{
			name:         "When the topic id is not good",
			searchString: "in:/wiki/topics/96a68720-1415-4e29-8c91-c9a65c516a05https://www.nytimes.com/2020/07/15/world/asia/china-trump-hong-kong.html",
			topicLimit:   1,
			linkLimit:    1,
		},
		{
			name:         "The limits are observed for when the topic is large",
			searchString: "in:/wiki/topics/33b18df0-eec8-11e8-b9e7-270ae3464cf5",
			topics:       condition(in.Exactly, 0),
			links:        condition(in.Exactly, 0),
			topicLimit:   0,
			linkLimit:    0,
		},
		{
			name:         "The limits are observed for when the topic is large",
			searchString: "in:/wiki/topics/33b18df0-eec8-11e8-b9e7-270ae3464cf5",
			topics:       condition(in.Exactly, 10),
			links:        condition(in.Exactly, 10),
			topicLimit:   10,
			linkLimit:    10,
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			query := queries.NewSearch(in.Everything, &td.searchString)

			topics, err := query.DescendantTopics(ctx, mutator.DB, td.topicLimit)
			in.Must(err)

			if !td.topics.Evaluate(len(topics)) {
				t.Fatalf(td.topics.Describe(len(topics)))
			}

			links, err := query.DescendantTopics(ctx, mutator.DB, td.linkLimit)
			in.Must(err)

			if !td.links.Evaluate(len(links)) {
				t.Fatalf(td.links.Describe(len(links)))
			}
		})
	}
}

func TestLinkDownSetAddedToNewParentTopic(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})

	mutator.DeleteLinksByURL("http://nytimely.com")
	mutator.DeleteTopicsByName("Topic 1", "Parent topic 1")

	topic := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Topic 1",
	})

	parentTopic := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Parent topic 1",
	})

	link := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{topic.ID},
		Title:          "New York Timely",
		URL:            "http://nytimely.com",
	})

	// The link is found under the topic
	query := queries.NewSearch(topic, &link.URL)
	links, err := query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) < 1 {
		t.Fatal("Expected at least one link")
	}

	// The link is not found under the (future) parent topic
	query = queries.NewSearch(parentTopic, &link.URL)
	links, err = query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) > 0 {
		t.Fatal("Expected the link not to be found under the future parent topic")
	}

	mutator.UpdateTopicParentTopics(in.UpdateTopicParentTopicsOptions{
		Topic:          topic,
		ParentTopicIds: []string{parentTopic.ID},
	})

	// Now that the parent topic is the parent of the other topic, the link should appear in searches of
	// the parent topic
	links, err = query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) < 1 {
		t.Fatal("Expected the link to be found under the parent topic now")
	}
}

func TestLinkDownSetRemovedFromRemovedParentTopic(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})

	mutator.DeleteLinksByURL("http://nytimely.com")
	mutator.DeleteTopicsByName("Topic 1", "Parent topic 1")

	parentTopic := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Parent topic 1",
	})

	topic := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{parentTopic.ID},
		Name:           "Topic 1",
	})

	link := mutator.UpsertLink(in.UpsertLinkOptions{
		ParentTopicIds: []string{topic.ID},
		Title:          "New York Timely",
		URL:            "http://nytimely.com",
	})

	// The link is found under the topic
	query := queries.NewSearch(topic, &link.URL)
	links, err := query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) < 1 {
		t.Fatal("Expected at least one link")
	}

	// The link is found under the parent topic
	query = queries.NewSearch(parentTopic, &link.URL)
	links, err = query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) < 1 {
		t.Fatal("Expected the link to be found under the parent topic")
	}

	mutator.UpdateTopicParentTopics(in.UpdateTopicParentTopicsOptions{
		Topic:          topic,
		ParentTopicIds: []string{in.Everything.ID},
	})

	// Now that the parent topic is no longer the parent of the other topic, the link should not appear in
	// searches of the parent topic
	query = queries.NewSearch(parentTopic, &link.URL)
	links, err = query.DescendantLinks(ctx, mutator.DB, 100)
	in.Must(err)

	if len(links) > 0 {
		t.Fatal("Expected the link to no longer be found under the parent topic")
	}
}

func TestDescendantTopicsLoadsOwner(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteTopicsByName("Topic 1", "Parent topic 1")

	parentTopic := mutator.UpsertTopic(in.UpsertTopicOptions{
		ParentTopicIds: []string{in.Everything.ID},
		Name:           "Parent topic 1",
	})

	searchString := "parent topic"
	query := queries.NewSearch(parentTopic, &searchString)
	topics, err := query.DescendantTopics(ctx, mutator.DB, 100)
	if err != nil {
		t.Fatalf("error: %s", err)
	}

	if len(topics) < 1 {
		t.Fatal("expected at least one topic")
	}

	if topics[0].GetRepo().GetOwner() == nil {
		t.Fatal("owner not loaded")
	}
}
