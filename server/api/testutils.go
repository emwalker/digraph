package api

import (
	"errors"
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
	Science          Topic
	Biology          Topic
	Chemistry        Topic
	UserData         map[string]interface{}
	OrganizationData map[string]interface{}
	TopicData        map[string]interface{}
)

type TestConnection struct {
	credentials *Credentials
	url         string
}

func (conn *TestConnection) Init() {
}

func getOrError(databaseId string, data map[string]interface{}, collection string) (interface{}, error) {
	object := data[databaseId]
	if object == nil {
		return nil, errors.New(fmt.Sprintf("%s not found: %s", collection, databaseId))
	}
	return object, nil
}

func (conn *TestConnection) GetOrganization(databaseId string) (interface{}, error) {
	return getOrError(databaseId, OrganizationData, "organization")
}

func (conn *TestConnection) GetUser(databaseId string) (interface{}, error) {
	return getOrError(databaseId, UserData, "user")
}

func (conn *TestConnection) GetTopic(databaseId string) (interface{}, error) {
	return getOrError(databaseId, TopicData, "topic")
}

func (conn *TestConnection) Viewer() (interface{}, error) {
	return &Gnusto, nil
}

func (conn *TestConnection) InsertUser(user *User) error {
	return nil
}

func (conn *TestConnection) RemoveUserByID(databaseId string) error {
	return nil
}

func (conn *TestConnection) SelectOrganizationTopics(
	dest *[]interface{},
	organization *Organization,
) error {
	*dest = append(*dest, []interface{}{Biology, Chemistry, Science}...)
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
	Science = Topic{
		OrganizationId: Tyrell.DatabaseId,
		DatabaseId:     "10",
		Description:    "Science",
	}
	Biology = Topic{
		OrganizationId: Tyrell.DatabaseId,
		DatabaseId:     "11",
		Description:    "Biology",
	}
	Chemistry = Topic{
		OrganizationId: Tyrell.DatabaseId,
		DatabaseId:     "12",
		Description:    "Chemistry",
	}
	UserData = map[string]interface{}{
		"10": &Gnusto,
		"11": &Frotz,
		"12": &Yomin,
		"13": &Bozbar,
		"14": &Rezrov,
	}
	OrganizationData = map[string]interface{}{
		"10": &Tyrell,
	}
	TopicData = map[string]interface{}{
		"10": &Science,
		"11": &Biology,
		"12": &Chemistry,
	}
}
