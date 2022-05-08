package activity_test

import (
	"testing"
	"time"

	"github.com/emwalker/digraph/golang/cmd/frontend/resolvers/activity"
)

func TestUpsertLinkLineItem(t *testing.T) {
	testCases := []struct {
		name        string
		data        activity.UpsertLink
		description string
	}{
		{
			name: "When there is no topic",
			data: activity.UpsertLink{
				CreatedAt: time.Now(),
				User:      activity.User{"Gnusto"},
				Link:      activity.Link{"Some URL", "http://www.some.url/"},
				Topics:    []activity.Topic{},
			},
			description: "Gnusto added [Some URL](http://www.some.url/)",
		},
		{
			name: "When there is a single topic",
			data: activity.UpsertLink{
				CreatedAt: time.Now(),
				User:      activity.User{"Gnusto"},
				Link:      activity.Link{"Some URL", "http://www.some.url/"},
				Topics: []activity.Topic{
					activity.Topic{"Topic 1", "1"},
				},
			},
			description: "Gnusto added [Some URL](http://www.some.url/) and tagged it with [Topic 1](/wiki/topics/1)",
		},
		{
			name: "When there are two topics",
			data: activity.UpsertLink{
				CreatedAt: time.Now(),
				User:      activity.User{"Gnusto"},
				Link:      activity.Link{"Some URL", "http://www.some.url/"},
				Topics: []activity.Topic{
					activity.Topic{"Topic 1", "1"},
					activity.Topic{"Topic 2", "2"},
				},
			},
			description: "Gnusto added [Some URL](http://www.some.url/) and tagged it with [Topic 1](/wiki/topics/1) and [Topic 2](/wiki/topics/2)",
		},
		{
			name: "When there are three topics",
			data: activity.UpsertLink{
				CreatedAt: time.Now(),
				User:      activity.User{"Gnusto"},
				Link:      activity.Link{"Some URL", "http://www.some.url/"},
				Topics: []activity.Topic{
					activity.Topic{"Topic 1", "1"},
					activity.Topic{"Topic 2", "2"},
					activity.Topic{"Topic 3", "3"},
				},
			},
			description: "Gnusto added [Some URL](http://www.some.url/) and tagged it with [Topic 1](/wiki/topics/1), [Topic 2](/wiki/topics/2) and [Topic 3](/wiki/topics/3)",
		},
		{
			name: "When there are newlines in the title",
			data: activity.UpsertLink{
				CreatedAt: time.Now(),
				User:      activity.User{"Gnusto"},
				Link:      activity.Link{"Some\nURL", "http://www.some.url/"},
				Topics:    []activity.Topic{},
			},
			description: "Gnusto added [Some URL](http://www.some.url/)",
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			edges, err := activity.MakeEdges([]activity.UpsertLink{td.data})
			if err != nil {
				t.Fatal(err)
			}

			if len(edges) < 1 {
				t.Fatal("Expected an edge")
			}

			node := edges[0].Node
			if node.Description != td.description {
				t.Fatalf("Expected '%s', got '%s", td.description, node.Description)
			}
		})
	}
}

func TestEscapeTitle(t *testing.T) {
	testCases := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "When there is a bracket in the title",
			input:    "The [title]",
			expected: "The \\[title\\]",
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			actual := activity.EscapeTitle(td.input)
			if actual != td.expected {
				t.Fatalf("Expected '%s', got '%s'", td.expected, actual)
			}
		})
	}
}

func TestEscapeURL(t *testing.T) {
	testCases := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "When there are parentheses in the url",
			input:    "http://some/url(with)parens",
			expected: "http://some/url%%28with%%29parens",
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			actual := activity.EscapeURL(td.input)
			if actual != td.expected {
				t.Fatalf("Expected '%s', got '%s'", td.expected, actual)
			}
		})
	}
}
