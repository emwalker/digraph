package api

import (
	"testing"

	"github.com/cayleygraph/cayley/quad"
	"github.com/cayleygraph/cayley/writer"
	"github.com/graphql-go/graphql"
	"github.com/labstack/echo"
)

func init() {
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
					organization(resourceId: "organization:tyrell") {
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
						"name": "Tyrell Corporation",
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
					topic(resourceId: "topic:science") {
						name
						description
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"topic": map[string]interface{}{
						"name":        "Science",
						"description": nil,
					},
				},
			},
		},
	}
}

var simpleGraph = []quad.Quad{
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:name"), "Tyrell Corporation", ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("rdf:type"), quad.IRI("foaf:Organization"), ""),
	quad.Make(quad.IRI("topic:science"), quad.IRI("di:name"), "Science", ""),
	quad.Make(quad.IRI("topic:science"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("topic:science"), ""),
	quad.Make(quad.IRI("topic:chemistry"), quad.IRI("di:name"), "Chemistry", ""),
	quad.Make(quad.IRI("topic:chemistry"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("topic:chemistry"), ""),
	quad.Make(quad.IRI("topic:biology"), quad.IRI("di:name"), "Biology", ""),
	quad.Make(quad.IRI("topic:biology"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("topic:biology"), ""),
	quad.Make(quad.IRI("topic:zoology"), quad.IRI("di:name"), "Zoology", ""),
	quad.Make(quad.IRI("topic:zoology"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), ""),
	quad.Make(quad.IRI("user:gnusto"), quad.IRI("di:name"), "Gnusto", ""),
	quad.Make(quad.IRI("user:gnusto"), quad.IRI("di:email"), "gnusto@tyrell.test", ""),
	quad.Make(quad.IRI("user:gnusto"), quad.IRI("rdf:type"), quad.IRI("foaf:Person"), ""),
}

func (conn *CayleyConnection) makeTestStore(data []quad.Quad) {
	writer, _ := writer.NewSingleReplication(conn.store, nil)
	for _, t := range data {
		writer.AddQuad(t)
	}
}

func TestQuery(t *testing.T) {
	conn := NewConnection("memstore", "")
	app, _ := New(conn, echo.New())
	conn.(*CayleyConnection).makeTestStore(simpleGraph)
	defer checkErr(conn.Close())

	for _, test := range Tests {
		params := graphql.Params{
			Schema:        *app.Schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
