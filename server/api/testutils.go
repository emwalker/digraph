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

	quad.Make(quad.IRI("link:github"), quad.IRI("di:title"), "Github", ""),
	quad.Make(quad.IRI("link:github"), quad.IRI("di:url"), "https://github.com", ""),
	quad.Make(quad.IRI("link:github"), quad.IRI("rdf:type"), quad.IRI("di:link"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("link:github"), ""),
	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("di:title"), "Wikipedia", ""),
	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("di:url"), "https://en.wikipedia.com", ""),
	quad.Make(quad.IRI("link:wikipedia"), quad.IRI("rdf:type"), quad.IRI("di:link"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("link:wikipedia"), ""),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("di:title"), "New York Times", ""),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("di:url"), "https://www.nytimes.com", ""),
	quad.Make(quad.IRI("link:nytimes"), quad.IRI("rdf:type"), quad.IRI("di:link"), ""),
	quad.Make(quad.IRI("organization:tyrell"), quad.IRI("di:owns"), quad.IRI("link:nytimes"), ""),
}

func (conn *CayleyConnection) makeTestStore(data []quad.Quad) {
	writer, _ := writer.NewSingleReplication(conn.store, nil)
	for _, t := range data {
		writer.AddQuad(t)
	}
}
