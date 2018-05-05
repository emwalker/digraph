package api

import (
	"reflect"
	"testing"

	"github.com/cayleygraph/cayley/quad"
	"github.com/cayleygraph/cayley/writer"
	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
)

type T struct {
	Query    string
	Schema   graphql.Schema
	Expected interface{}
}

var (
	Tests = []T{}

	urlTitles = map[string]string{
		"https://gnusto.test": "Gnusto's Homepage",
	}
)

func testTitleFetcher(url string) (string, error) {
	return urlTitles[url], nil
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

var org = quad.IRI("organization:tyrell")

var simpleGraph = []quad.Quad{
	quad.Make(org, quad.IRI("di:name"), "Tyrell Corporation", ""),
	quad.Make(org, quad.IRI("rdf:type"), quad.IRI("foaf:Organization"), ""),

	quad.Make(quad.IRI("topic:science"), quad.IRI("di:name"), "Science", org),
	quad.Make(quad.IRI("topic:science"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), org),
	quad.Make(quad.IRI("topic:chemistry"), quad.IRI("di:name"), "Chemistry", org),
	quad.Make(quad.IRI("topic:chemistry"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), org),
	quad.Make(quad.IRI("topic:biology"), quad.IRI("di:name"), "Biology", org),
	quad.Make(quad.IRI("topic:biology"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), org),
	quad.Make(quad.IRI("topic:zoology"), quad.IRI("di:name"), "Zoology", ""),
	quad.Make(quad.IRI("topic:zoology"), quad.IRI("rdf:type"), quad.IRI("foaf:topic"), ""),

	quad.Make(quad.IRI("user:gnusto"), quad.IRI("di:name"), "Gnusto", ""),
	quad.Make(quad.IRI("user:gnusto"), quad.IRI("di:email"), "gnusto@tyrell.test", ""),
	quad.Make(quad.IRI("user:gnusto"), quad.IRI("rdf:type"), quad.IRI("foaf:Person"), ""),

	quad.Make(quad.IRI("link:github"), quad.IRI("di:title"), "Github", org),
	quad.Make(quad.IRI("link:github"), quad.IRI("di:url"), "https://github.com", org),
	quad.Make(quad.IRI("link:github"), quad.IRI("rdf:type"), quad.IRI("di:link"), org),
	quad.Make(quad.IRI("topic:biology"), quad.IRI("di:includes"), quad.IRI("link:github"), org),
	quad.Make(quad.IRI("topic:chemistry"), quad.IRI("di:includes"), quad.IRI("link:github"), org),

	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("di:title"), "Wikipedia", org),
	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("di:url"), "https://en.wikipedia.com", org),
	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("rdf:type"), quad.IRI("di:link"), org),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("di:title"), "New York Times", org),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("di:url"), "https://www.nytimes.com", org),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("rdf:type"), quad.IRI("di:link"), org),
}

func (conn *CayleyConnection) makeTestStore(data []quad.Quad) {
	writer, _ := writer.NewSingleReplication(conn.store, nil)
	for _, t := range data {
		writer.AddQuad(t)
	}
}
