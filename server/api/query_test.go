package api

import (
	"testing"

	"github.com/graphql-go/graphql"
)

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
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"viewer": map[string]interface{}{
						"name":  "Gnusto",
						"email": "gnusto@tyrell.test",
					},
				},
			},
		},
		{
			Query: `
				query {
					organization(id: "1234") {
						name
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"organization": map[string]interface{}{
						"name": "Tyrell Corporation",
					},
				},
			},
		},
	}
}

func TestQuery(t *testing.T) {
	for _, test := range Tests {
		params := graphql.Params{
			Schema:        schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
