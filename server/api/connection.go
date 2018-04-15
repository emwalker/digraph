package api

import (
	"fmt"
	"log"
)

type Connection interface {
	GetUserByID(id string) (*User, error)
	Init()
	InsertUser(user *User) error
	RemoveUserByID(id string) error
}

func NewConnection(driverName string, url string) Connection {
	switch {
	case driverName == "postgres":
		return &PostgresConnection{url: url}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", driverName))
	}
	return nil
}
