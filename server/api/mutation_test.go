package api

import (
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
)

var testIndex int

// map field to `theNumber` so it can be resolve by the default ResolveFn
type testNumberHolder struct {
	TheNumber int `json:"theNumber"`
}

type testRoot struct {
	NumberHolder *testNumberHolder
}

func newTestRoot(originalNumber int) *testRoot {
	return &testRoot{
		NumberHolder: &testNumberHolder{originalNumber},
	}
}

func testMutations(t *testing.T, doc string, expected *graphql.Result) {
	app, _ := New(&Config{
		DriverName: "memstore",
		FetchTitle: testTitleFetcher,
	})
	app.Connection.(*CayleyConnection).makeTestStore(simpleGraph)

	params := graphql.ExecuteParams{
		Schema: *app.Schema,
		AST:    testutil.TestParse(t, doc),
		Root:   newTestRoot(testIndex),
	}
	testIndex += 1

	result := testutil.TestExecute(t, params)
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("Unexpected result, Diff: %v", testutil.Diff(expected, result))
	}
}

func TestCreateTopic(t *testing.T) {
	doc := `
	mutation M {
		first: createTopic(
			input: {
				organizationResourceId: "organization:tyrell",
				name: "Gnusto",
				description: "Things about Gnusto",
			}
		) {
			topicEdge {
				node {
					name
					description
				}
			}
		},
		second: createTopic(
			input: {
				organizationResourceId: "organization:tyrell",
				name: "Yomin",
			}
		) {
			topicEdge {
				node {
					name
					description
				}
			}
		}
	}`

	expected := &graphql.Result{
		Data: map[string]interface{}{
			"first": map[string]interface{}{
				"topicEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"name":        "Gnusto",
						"description": "Things about Gnusto",
					},
				},
			},
			"second": map[string]interface{}{
				"topicEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"name":        "Yomin",
						"description": nil,
					},
				},
			},
		},
	}

	testMutations(t, doc, expected)
}

func TestSelectTopic(t *testing.T) {
	doc := `
	mutation M {
		first: selectTopic(
			input: {
				topicId: "topic:science",
			}
		) {
			topic {
				name
			}
		},
		second: selectTopic(
			input: {
				topicId: "does-not-exist",
			}
		) {
			topic {
				name
			}
		}
	}`

	expected := &graphql.Result{
		Data: map[string]interface{}{
			"first": map[string]interface{}{
				"topic": map[string]interface{}{
					"name": "Science",
				},
			},
			"second": map[string]interface{}{
				"topic": nil,
			},
		},
	}

	testMutations(t, doc, expected)
}

func TestCreateLink(t *testing.T) {
	doc := `
	mutation M {
		first: createLink(
			input: {
				organizationResourceId: "organization:tyrell",
				url: "https://gnusto.test",
			}
		) {
			linkEdge {
				node {
					title
					url
				}
			}
		}
	}`

	expected := &graphql.Result{
		Data: map[string]interface{}{
			"first": map[string]interface{}{
				"linkEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"title": "Gnusto's Homepage",
						"url":   "https://gnusto.test",
					},
				},
			},
		},
	}

	testMutations(t, doc, expected)
}
