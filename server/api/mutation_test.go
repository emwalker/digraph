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
				organizationId: "organization:tyrell",
				name: "Gnusto",
				description: "Things about Gnusto",
				topicIds: ["topic:science"],
			}
		) {
			topicEdge {
				node {
					name
					description

					parentTopics {
						edges {
							node {
								name
							}
						}
					}
				}
			}
		},
		second: createTopic(
			input: {
				organizationId: "organization:tyrell",
				name: "Yomin",
			}
		) {
			topicEdge {
				node {
					name
					description

					parentTopics {
						edges {
							node {
								name
							}
						}
					}
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
						"parentTopics": map[string]interface{}{
							"edges": []interface{}{
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Science",
									},
								},
							},
						},
					},
				},
			},
			"second": map[string]interface{}{
				"topicEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"name":        "Yomin",
						"description": nil,
						"parentTopics": map[string]interface{}{
							"edges": []interface{}{
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Root",
									},
								},
							},
						},
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
				organizationId: "organization:tyrell",
				topicId: "topic:science",
			}
		) {
			topic {
				name
			}
		},
		second: selectTopic(
			input: {
				organizationId: "organization:tyrell",
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

func TestUpsertLink(t *testing.T) {
	doc := `
	mutation M {
		first: upsertLink(
			input: {
				organizationId: "organization:tyrell",
				url: "https://github.com",
				addTopicIds: [
					"topic:science",
				],
			}
		) {
			linkEdge {
				node {
					title
					url
					topics {
						edges {
							node {
								name
							}
						}
					}
				}
			}
		},

		second: upsertLink(
			input: {
				organizationId: "organization:tyrell",
				resourceId: "link:wikipedia",
				title: "Frotz",
				url: "https://frotz.test",
			}
		) {
			linkEdge {
				node {
					title
					url
				}
			}
		},
	}`

	expected := &graphql.Result{
		Data: map[string]interface{}{
			"first": map[string]interface{}{
				"linkEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"title": "Github",
						"url":   "https://github.com",
						"topics": map[string]interface{}{
							"edges": []interface{}{
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Biology",
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Chemistry",
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Science",
									},
								},
							},
						},
					},
				},
			},

			"second": map[string]interface{}{
				"linkEdge": map[string]interface{}{
					"node": map[string]interface{}{
						"title": "Frotz",
						"url":   "https://frotz.test",
					},
				},
			},
		},
	}

	testMutations(t, doc, expected)
}
