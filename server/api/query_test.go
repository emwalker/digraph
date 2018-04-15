package api

import (
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
)

type T struct {
	Query    string
	Schema   graphql.Schema
	Expected interface{}
}

var Tests = []T{}

func init() {
	Init(NewConnection("test", "some://url"))

	Tests = []T{
		{
			Query: `
        query {
          viewer {
            name
            email
          }
        }
      `,
			Schema: schema,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"viewer": map[string]interface{}{
						"name":  "Gnusto",
						"email": "gnusto@tyrell.test",
					},
				},
			},
		},
	}
}

func TestQuery(t *testing.T) {
	for _, test := range Tests {
		params := graphql.Params{
			Schema:        test.Schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}

func testGraphql(test T, p graphql.Params, t *testing.T) {
	result := graphql.Do(p)
	if len(result.Errors) > 0 {
		t.Fatalf("wrong result, unexpected errors: %v", result.Errors)
	}
	if !reflect.DeepEqual(result, test.Expected) {
		t.Fatalf(
			"wrong result, query: %v, graphql result diff: %v",
			test.Query,
			testutil.Diff(test.Expected, result),
		)
	}
}
