package api

import (
	"errors"
	"fmt"

	"github.com/graphql-go/relay"
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

func (conn *PostgresConnection) GetByKey(
	object Resource,
	collection string,
	sql string,
	key string,
) (interface{}, error) {
	err := conn.db.Get(object, sql, key)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("%s not found: %s", collection, key))
	}
	object.Init()
	return object, nil
}

func (conn *PostgresConnection) Get(
	object Resource,
	collection string,
	sql string,
	id string,
) (interface{}, error) {
	databaseId := relay.FromGlobalID(id)
	return conn.GetByKey(object, collection, sql, databaseId.ID)
}

func (conn *PostgresConnection) GetUser(id string) (interface{}, error) {
	return conn.Get(
		&User{},
		"User",
		"select *, id as databaseId from users where id = $1",
		id,
	)
}

func (conn *PostgresConnection) GetOrganization(id string) (interface{}, error) {
	return conn.Get(
		&Organization{},
		"Organization",
		"select *, id as databaseId from organizations where id = $1",
		id,
	)
}

func (conn *PostgresConnection) GetTopic(id string) (interface{}, error) {
	return conn.Get(
		&Topic{},
		"Topic",
		"select *, id as databaseId from topics where id = $1",
		id,
	)
}

func (conn *PostgresConnection) Viewer() (interface{}, error) {
	object, err := conn.GetByKey(
		&User{},
		"User",
		"select *, id as databaseId from users where email like $1",
		Gnusto.Email,
	)

	if err != nil {
		return nil, err
	}

	object.(Resource).Init()
	return object, nil
}

func (conn *PostgresConnection) SelectOrganizationTopics(
	dest *[]interface{},
	organization *Organization,
) error {
	var topics = []Topic{}
	err := conn.db.Select(
		&topics,
		"select *, id as databaseId from topics where organization_id = $1",
		organization.DatabaseID,
	)

	if err != nil {
		return errors.New(fmt.Sprintf("there was a problem: %s", err))
	}

	for _, topic := range topics {
		topic.Init()
		*dest = append(*dest, topic)
	}
	return nil
}
