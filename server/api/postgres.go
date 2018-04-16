package api

import (
	"database/sql"
	"fmt"
	"log"
)

type PostgresConnection struct {
	credentials *Credentials
	db          *sql.DB
	url         string
}

func (conn *PostgresConnection) Init() {
	var err error
	db, err := sql.Open("postgres", conn.url)
	if err != nil {
		log.Fatal(err)
	}
	conn.db = db
}

func (conn *PostgresConnection) FindUser(databaseId string) (*User, error) {
	var email string
	err := conn.db.QueryRow("select email from users where id = $1", databaseId).Scan(&email)
	if err != nil {
		return nil, Error{
			Message:       fmt.Sprintf("user not found: %s", databaseId),
			OriginalError: err,
		}
	}
	return &User{
		ID:         databaseId,
		DatabaseId: databaseId,
		Email:      email,
	}, nil
}

func (conn *PostgresConnection) FindOrganization(databaseId string) (*Organization, error) {
	var name string
	err := conn.db.QueryRow("select name from organizations where id = $1", databaseId).Scan(&name)
	if err != nil {
		return nil, Error{
			Message:       fmt.Sprintf("organization not found: %s", databaseId),
			OriginalError: err,
		}
	}
	return &Organization{
		ID:         databaseId,
		DatabaseId: databaseId,
		Name:       name,
	}, nil
}

func (conn *PostgresConnection) GetViewer() (*User, error) {
	return &Gnusto, nil
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
