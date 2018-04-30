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
	CreateLink(string, string, string) (*Link, error)
	CreateTopic(string, string, *string) (*Topic, error)
	FetchLink(string) (interface{}, error)
	FetchLinks(*[]interface{}, *Organization) error
	FetchOrganization(string) (interface{}, error)
	FetchTopic(string) (interface{}, error)
	FetchTopics(*[]interface{}, *Organization) error
	FetchUser(string) (interface{}, error)
	Init() error
	Viewer() (interface{}, error)
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
