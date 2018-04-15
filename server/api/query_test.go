package api

import (
	"testing"

	"github.com/graphql-go/graphql"
)

func init() {
	connection := NewConnection(
		&Credentials{BearerToken: "1234"},
		"test",
		"some://url",
	)
	Init(connection)

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
					organization(id: "10") {
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
			Schema:        schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
