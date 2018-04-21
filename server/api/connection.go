package api

import (
	"fmt"
	"log"
)

type Credentials struct {
	BearerToken string
}

type Error struct {
	Message       string
	OriginalError error
}

func (e Error) Error() string {
	return fmt.Sprintf("%v", e.Message)
}

type Connection interface {
	GetOrganization(string) (interface{}, error)
	GetTopic(string) (interface{}, error)
	GetUser(string) (interface{}, error)
	Viewer() (interface{}, error)
	Init() error
	SelectOrganizationTopics(*[]interface{}, *Organization) error
}

func NewConnection(driverName string, url string) Connection {
	switch driverName {
	case "postgres":
		return &PostgresConnection{url: url}
	case "test":
		return &TestConnection{url: url}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", driverName))
	}
	return nil
}
