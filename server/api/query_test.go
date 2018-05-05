package api

import (
	"testing"

	"github.com/graphql-go/graphql"
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

						topic(resourceId: "topic:biology") {
							name
							description
							resourcePath

							parentTopics {
								edges {
									node {
										name
									}
								}
							}

							childTopics {
								edges {
									node {
										name
									}
								}
							}

							links {
								edges {
									node {
										title
									}
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
										"name": "Root",
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"name": "Science",
									},
								},
							},
						},
						"topic": map[string]interface{}{
							"name":         "Biology",
							"description":  nil,
							"resourcePath": "/topics/biology",
							"parentTopics": map[string]interface{}{
								"edges": []interface{}{
									map[string]interface{}{
										"node": map[string]interface{}{
											"name": "Science",
										},
									},
								},
							},
							"childTopics": map[string]interface{}{
								"edges": []interface{}{},
							},
							"links": map[string]interface{}{
								"edges": []interface{}{
									map[string]interface{}{
										"node": map[string]interface{}{
											"title": "Github",
										},
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
					organization(resourceId: "organization:tyrell") {
						links(first: 100) {
							edges {
								node {
									title
									url

									topics(first: 100) {
										edges {
											node {
												name
												resourceId
											}
										}
									}
								}
							}
						}
					}
				}
			`,
			Expected: &graphql.Result{
				Data: map[string]interface{}{
					"organization": map[string]interface{}{
						"links": map[string]interface{}{
							"edges": []interface{}{
								map[string]interface{}{
									"node": map[string]interface{}{
										"title": "Github",
										"url":   "https://github.com",
										"topics": map[string]interface{}{
											"edges": []interface{}{
												map[string]interface{}{
													"node": map[string]interface{}{
														"name":       "Biology",
														"resourceId": "topic:biology",
													},
												},
												map[string]interface{}{
													"node": map[string]interface{}{
														"name":       "Chemistry",
														"resourceId": "topic:chemistry",
													},
												},
											},
										},
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"title": "New York Times",
										"url":   "https://www.nytimes.com",
										"topics": map[string]interface{}{
											"edges": []interface{}{},
										},
									},
								},
								map[string]interface{}{
									"node": map[string]interface{}{
										"title": "Wikipedia",
										"url":   "https://en.wikipedia.com",
										"topics": map[string]interface{}{
											"edges": []interface{}{},
										},
									},
								},
							},
						},
					},
				},
			},
		},
	}
}

func TestQuery(t *testing.T) {
	app, _ := New(&Config{
		DriverName: "memstore",
		FetchTitle: testTitleFetcher,
	})
	app.Connection.(*CayleyConnection).makeTestStore(simpleGraph)

	for _, test := range Tests {
		params := graphql.Params{
			Schema:        *app.Schema,
			RequestString: test.Query,
		}
		testGraphql(test, params, t)
	}
}
