package api

import (
	"testing"

	"github.com/graphql-go/graphql"
)

var app *App

func init() {
	conn := NewConnection("test", "some://url")
	app, _ = New(conn)

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
					organization(id: "T3JnYW5pemF0aW9uOjEw") {
						name

						topics(first: 100) {
							edges {
								node {
									name
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
		},
		{
			Query: `
				query {
					topic(id: "VG9waWM6MTA=") {
						name
						description
						resourcePath
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"topic": map[string]interface{}{
						"name":         Science.Name,
						"description":  *Science.Description,
						"resourcePath": Science.ResourcePath,
					},
				},
			},
		},
	}
}

func TestQuery(t *testing.T) {
	for _, test := range Tests {
		params := graphql.Params{
			Schema:        *app.Schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
