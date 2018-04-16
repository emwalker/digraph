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
	FindOrganization(id string) (*Organization, error)
	FindUser(id string) (*User, error)
	GetViewer() (*User, error)
	Init()
	InsertUser(user *User) error
	RemoveUserByID(id string) error
}

func NewConnection(credentials *Credentials, driverName string, url string) Connection {
	switch driverName {
	case "postgres":
		return &PostgresConnection{credentials: credentials, url: url}
	case "test":
		return &TestConnection{credentials: credentials, url: url}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", driverName))
	}
	return nil
}
