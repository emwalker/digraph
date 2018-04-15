package api

import (
	"fmt"
	"log"
)

type Connection interface {
	GetOrganizationByID(id string) (*Organization, error)
	GetUserByID(id string) (*User, error)
	GetViewer() (*User, error)
	Init()
	InsertUser(user *User) error
	RemoveUserByID(id string) error
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
