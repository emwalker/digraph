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
	FetchTitle(string) (string, error)
	FetchTopic(string) (interface{}, error)
	FetchTopics(*[]interface{}, *Organization) error
	FetchUser(string) (interface{}, error)
	Init() error
	Viewer() (interface{}, error)
}

func (config *Config) newConnection() Connection {
	switch config.DriverName {
	case "postgres", "memstore":
		return &CayleyConnection{
			address:     config.Address,
			driverName:  config.DriverName,
			titleForUrl: config.FetchTitle,
		}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", config.DriverName))
	}
	return nil
}
