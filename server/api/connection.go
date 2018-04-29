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
	Close() error
	CreateTopic(string, string, *string) (*Topic, error)
	GetOrganization(string) (interface{}, error)
	GetTopic(string) (interface{}, error)
	GetUser(string) (interface{}, error)
	Viewer() (interface{}, error)
	Init() error
	FetchLinks(*[]interface{}, *Organization) error
	FetchTopics(*[]interface{}, *Organization) error
}

func NewConnection(driverName string, address string) Connection {
	switch driverName {
	case "postgres":
		return &CayleyConnection{address: address, driverName: driverName}
	case "memstore":
		return &CayleyConnection{address: address, driverName: driverName}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", driverName))
	}
	return nil
}
