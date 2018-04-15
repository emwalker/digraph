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

func (conn *PostgresConnection) GetUserByID(id string) (*User, error) {
	var email string
	err := conn.db.QueryRow("select email from users where id = $1", id).Scan(&email)
	if err != nil {
		return nil, Error{
			Message:       fmt.Sprintf("user not found: %s", id),
			OriginalError: err,
		}
	}
	return &User{
		ID:    id,
		Email: email,
	}, nil
}

func (conn *PostgresConnection) GetOrganizationByID(id string) (*Organization, error) {
	var name string
	err := conn.db.QueryRow("select name from organizations where id = $1", id).Scan(&name)
	if err != nil {
		return nil, Error{
			Message:       fmt.Sprintf("organization not found: %s", id),
			OriginalError: err,
		}
	}
	return &Organization{
		ID:   id,
		Name: name,
	}, nil
}

func (conn *PostgresConnection) GetViewer() (*User, error) {
	return &Gnusto, nil
}

func (conn *PostgresConnection) InsertUser(user *User) error {
	var id string
	err := conn.db.QueryRow(`
    INSERT INTO users(email)
    VALUES ($1)
    RETURNING id
  `, user.Email).Scan(&id)
	if err != nil {
		return err
	}
	user.ID = id
	return nil
}

func (conn *PostgresConnection) RemoveUserByID(id string) error {
	_, err := conn.db.Exec("DELETE FROM users WHERE id=$1", id)
	return err
}
