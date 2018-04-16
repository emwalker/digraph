package api

import (
	"testing"

	"github.com/graphql-go/graphql"
)

func init() {
	Init(
		NewConnection(
			&Credentials{BearerToken: "1234"},
			"test",
			"some://url",
		),
	)

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
						"name":  Gnusto.Name,
						"email": Gnusto.Email,
					},
				},
			},
		},
		{
			Query: `
				query {
					organization(databaseId: "10") {
						name
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"organization": map[string]interface{}{
						"name": Tyrell.Name,
					},
				},
			},
		},
	}
}

func TestQuery(t *testing.T) {
	for _, test := range Tests {
		params := graphql.Params{
			Schema:        Schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
