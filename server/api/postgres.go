package api

import (
	"errors"
	"fmt"
	"log"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

type PostgresConnection struct {
	credentials *Credentials
	db          *sqlx.DB
	url         string
}

func (conn *PostgresConnection) Init() {
	var err error
	db, err := sqlx.Connect("postgres", conn.url)
	if err != nil {
		log.Fatal(err)
	}
	conn.db = db
}

func (conn *PostgresConnection) GetUser(databaseId string) (*User, error) {
	var user = User{}
	err := conn.db.Get(&user, "select *, id as databaseId from users where id = $1", databaseId)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("user not found: %s", databaseId))
	}
	return &user, nil
}

func (conn *PostgresConnection) GetOrganization(databaseId string) (*Organization, error) {
	var organization = Organization{}
	err := conn.db.Get(
		&organization,
		"select *, id as databaseId from organizations where id = $1",
		databaseId,
	)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("organization not found: %s", databaseId))
	}
	return &organization, nil
}

func (conn *PostgresConnection) Viewer() (*User, error) {
	var user = User{}
	err := conn.db.Get(&user, "select *, id as databaseId from users where email like $1", Gnusto.Email)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("user not found: %s", Gnusto.Email))
	}
	return &user, nil
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
