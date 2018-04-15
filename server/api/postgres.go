package api

import (
	"database/sql"
	"log"
)

type PostgresConnection struct {
	db  *sql.DB
	url string
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
	err := conn.db.QueryRow("select email from users where id=$1", id).Scan(&email)
	if err != nil {
		return nil, err
	}
	return &User{
		ID:    id,
		Email: email,
	}, nil
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
