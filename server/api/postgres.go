package api

import (
	"database/sql"

	"github.com/laurent22/go-sqlkv"
	_ "github.com/lib/pq"
)

type pgSessionStore struct {
	driverName string
	address    string
	store      *sqlkv.SqlKv
}

func newPgSessionStore(driverName string, address string) *pgSessionStore {
	db, err := sql.Open(driverName, address)
	checkErr(err)
	store := sqlkv.New(db, "kvstore")
	store.SetDriverName(driverName)

	return &pgSessionStore{
		driverName: driverName,
		address:    address,
		store:      store,
	}
}

func (s *pgSessionStore) Get(userId string, key string) (string, error) {
	return s.store.String(userId + ":" + key), nil
}

func (s *pgSessionStore) Set(userId string, key string, value string) error {
	s.store.SetString(userId+":"+key, value)
	return nil
}
