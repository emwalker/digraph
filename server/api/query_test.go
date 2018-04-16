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

						topics(first: 100) {
							edges {
								node {
									description
								}
							}
						}
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"organization": map[string]interface{}{
						"name": Tyrell.Name,
						"topics": map[string]interface{}{
							"edges": []interface{}{
								map[string]interface{}{
									"node": map[string]interface{}{
										"description": "Biology",
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"description": "Chemistry",
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"description": "Science",
									},
								},
							},
						},
					},
				},
			},
		},
		{
			Query: `
				query {
					topic(databaseId: "10") {
						description
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"topic": map[string]interface{}{
						"description": Science.Description,
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
