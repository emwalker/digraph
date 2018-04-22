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
