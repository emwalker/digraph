package api

import (
	"errors"
	"fmt"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

type PostgresConnection struct {
	db  *sqlx.DB
	url string
}

func (conn *PostgresConnection) Init() error {
	var err error
	db, err := sqlx.Connect("postgres", conn.url)
	if err != nil {
		return err
	}
	conn.db = db
	return nil
}

func (conn *PostgresConnection) getOrError(
	object interface{},
	collection string,
	sql string,
	key string,
) (interface{}, error) {
	err := conn.db.Get(object, sql, key)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("%s not found: %s", collection, key))
	}
	return object, nil
}

func (conn *PostgresConnection) GetUser(databaseId string) (interface{}, error) {
	return conn.getOrError(
		&User{},
		"user",
		"select *, id as databaseId from users where id = $1",
		databaseId,
	)
}

func (conn *PostgresConnection) GetOrganization(databaseId string) (interface{}, error) {
	return conn.getOrError(
		&Organization{},
		"organization",
		"select *, id as databaseId from organizations where id = $1",
		databaseId,
	)
}

func (conn *PostgresConnection) GetTopic(databaseId string) (interface{}, error) {
	return conn.getOrError(
		&Topic{},
		"topic",
		"select *, id as databaseId from topics where id = $1",
		databaseId,
	)
}

func (conn *PostgresConnection) Viewer() (interface{}, error) {
	return conn.getOrError(
		&User{},
		"user",
		"select *, id as databaseId from users where email like $1",
		Gnusto.Email,
	)
}

func (conn *PostgresConnection) InsertUser(user *User) error {
	var databaseId string
	err := conn.db.QueryRow(`
    INSERT INTO users(email)
    VALUES ($1)
    RETURNING id
  `, user.Email).Scan(&databaseId)
	if err != nil {
		return err
	}
	user.DatabaseId = databaseId
	return nil
}

func (conn *PostgresConnection) RemoveUserByID(databaseId string) error {
	_, err := conn.db.Exec("DELETE FROM users WHERE id = $1", databaseId)
	return err
}

func (conn *PostgresConnection) SelectOrganizationTopics(
	dest *[]interface{},
	organization *Organization,
) error {
	var topics = []Topic{}
	err := conn.db.Select(
		&topics,
		"select *, id as databaseId from topics where organization_id = $1",
		organization.DatabaseId,
	)

	if err != nil {
		return errors.New(fmt.Sprintf("there was a problem: %s", err))
	}

	for _, topic := range topics {
		*dest = append(*dest, topic)
	}
	return nil
}
