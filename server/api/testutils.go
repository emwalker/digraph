package api

import (
	"errors"
	"fmt"
	"reflect"
	"testing"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/testutil"
	"github.com/graphql-go/relay"
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
	url string
}

func (conn *TestConnection) Init() error {
	return nil
}

func getOrError(
	id string,
	data map[string]interface{},
	collection string,
) (interface{}, error) {
	databaseId := relay.FromGlobalID(id)
	object := data[databaseId.ID].(Resource)
	if object == nil {
		return nil, errors.New(fmt.Sprintf("%s not found: %s", collection, databaseId))
	}
	object.Init()
	return object, nil
}

func (conn *TestConnection) GetOrganization(id string) (interface{}, error) {
	return getOrError(id, OrganizationData, "Organization")
}

func (conn *TestConnection) GetUser(id string) (interface{}, error) {
	return getOrError(id, UserData, "User")
}

func (conn *TestConnection) GetTopic(id string) (interface{}, error) {
	return getOrError(id, TopicData, "Topic")
}

func (conn *TestConnection) Viewer() (interface{}, error) {
	return &Gnusto, nil
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
		DatabaseID: "10",
		Name:       "Gnusto",
		Email:      "gnusto@tyrell.test",
	}
	Frotz = User{
		DatabaseID: "11",
		Name:       "Frotz",
		Email:      "frotz@tyrell.test",
	}
	Yomin = User{
		DatabaseID: "12",
		Name:       "Yomin",
		Email:      "yomin@tyrell.test",
	}
	Bozbar = User{
		DatabaseID: "13",
		Name:       "Bozbar",
		Email:      "bozbar@tyrell.test",
	}
	Rezrov = User{
		DatabaseID: "14",
		Name:       "Rezrov",
		Email:      "rezrov@tyrell.test",
	}
	Tyrell = Organization{
		DatabaseID: "10",
		Name:       "Tyrell Corporation",
	}
	var description = "One of the branches of knowledge"
	Science = Topic{
		OrganizationDatabaseID: Tyrell.DatabaseID,
		DatabaseID:             "10",
		Name:                   "Science",
		Description:            &description,
		ResourcePath:           "/topics/VG9waWM6MTA=",
	}
	Biology = Topic{
		OrganizationDatabaseID: Tyrell.DatabaseID,
		DatabaseID:             "11",
		Name:                   "Biology",
		ResourcePath:           "/topics/VG9waWM6MTE=",
	}
	Chemistry = Topic{
		OrganizationDatabaseID: Tyrell.DatabaseID,
		DatabaseID:             "12",
		Name:                   "Chemistry",
		ResourcePath:           "/topics/VG9waWM6MTI=",
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
