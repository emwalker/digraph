package api

import (
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
	"github.com/labstack/echo"
)

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

func TestMutations_CreateTopic(t *testing.T) {
	conn := NewConnection("memstore", "")
	app, _ := New(conn, echo.New())
	conn.(*CayleyConnection).makeTestStore(simpleGraph)
	defer checkErr(conn.Close())

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

	params := graphql.ExecuteParams{
		Schema: *app.Schema,
		AST:    testutil.TestParse(t, doc),
		Root:   newTestRoot(1),
	}

	result := testutil.TestExecute(t, params)
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("Unexpected result, Diff: %v", testutil.Diff(expected, result))
	}
}
