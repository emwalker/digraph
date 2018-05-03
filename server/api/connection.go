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

type SessionStore interface {
	Set(string, string, string) error
	Get(string, string) (string, error)
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
	SelectTopic(string, string) (*Topic, error)
	SelectedTopic(string) (*Topic, error)
	Viewer() (interface{}, error)
}

type memstoreSessionStore struct {
	data map[string]string
}

func newMemstoreSessionStore() *memstoreSessionStore {
	return &memstoreSessionStore{
		data: map[string]string{},
	}
}

func (s *memstoreSessionStore) Set(userId string, key string, value string) error {
	s.data[key] = value
	return nil
}

func (s *memstoreSessionStore) Get(userId string, key string) (string, error) {
	return s.data[key], nil
}

func (config *Config) sessionStore() SessionStore {
	switch config.DriverName {
	case "postgres":
		return newPgSessionStore(config.DriverName, config.Address)
	case "memstore":
		return newMemstoreSessionStore()
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", config.DriverName))
	}
	return nil
}

func (config *Config) newConnection() Connection {
	switch config.DriverName {
	case "postgres", "memstore":
		return &CayleyConnection{
			address:     config.Address,
			driverName:  config.DriverName,
			titleForUrl: config.FetchTitle,
			session:     config.sessionStore(),
		}
	default:
		log.Fatal(fmt.Sprintf("do not recognize driver: %s", config.DriverName))
	}
	return nil
}
