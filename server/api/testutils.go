package api

import (
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
)

type TestConnection struct {
	url string
}

func (conn *TestConnection) Init() {
}

func (conn *TestConnection) GetOrganizationByID(id string) (*Organization, error) {
	return &Organization{
		ID:   id,
		Name: "Tyrell Corporation",
	}, nil
}

func (conn *TestConnection) GetUserByID(id string) (*User, error) {
	return &User{
		ID:    id,
		Email: "some@email.test",
	}, nil
}

func (conn *TestConnection) GetViewer() (*User, error) {
	return &User{
		ID:    "1234",
		Name:  "Gnusto",
		Email: "gnusto@tyrell.test",
	}, nil
}

func (conn *TestConnection) InsertUser(user *User) error {
	return nil
}

func (conn *TestConnection) RemoveUserByID(id string) error {
	return nil
}

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
