package api

import (
	"fmt"
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
)

var (
	Gnusto           User
	Frotz            User
	Yomin            User
	Bozbar           User
	Rezrov           User
	Tyrell           Organization
	UserData         map[string]*User
	OrganizationData map[string]*Organization
)

type TestConnection struct {
	credentials *Credentials
	url         string
}

func (conn *TestConnection) Init() {
}

func (conn *TestConnection) FindOrganization(databaseId string) (*Organization, error) {
	organization := OrganizationData[databaseId]
	if organization == nil {
		return nil, Error{Message: fmt.Sprintf("organization not found: %s", databaseId)}
	}
	return organization, nil
}

func (conn *TestConnection) FindUser(databaseId string) (*User, error) {
	user := UserData[databaseId]
	if user == nil {
		return nil, Error{Message: fmt.Sprintf("user not found: %s", databaseId)}
	}
	return user, nil
}

func (conn *TestConnection) GetViewer() (*User, error) {
	return &Gnusto, nil
}

func (conn *TestConnection) InsertUser(user *User) error {
	return nil
}

func (conn *TestConnection) RemoveUserByID(databaseId string) error {
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

func init() {
	Gnusto = User{
		DatabaseId: "10",
		Name:       "Gnusto",
		Email:      "gnusto@tyrell.test",
	}
	Frotz = User{
		DatabaseId: "11",
		Name:       "Frotz",
		Email:      "frotz@tyrell.test",
	}
	Yomin = User{
		DatabaseId: "12",
		Name:       "Yomin",
		Email:      "yomin@tyrell.test",
	}
	Bozbar = User{
		DatabaseId: "13",
		Name:       "Bozbar",
		Email:      "bozbar@tyrell.test",
	}
	Rezrov = User{
		DatabaseId: "14",
		Name:       "Rezrov",
		Email:      "rezrov@tyrell.test",
	}
	Tyrell = Organization{
		DatabaseId: "10",
		Name:       "Tyrell Corporation",
	}
	UserData = map[string]*User{
		"10": &Gnusto,
		"11": &Frotz,
		"12": &Yomin,
		"13": &Bozbar,
		"14": &Rezrov,
	}
	OrganizationData = map[string]*Organization{
		"10": &Tyrell,
	}
}
