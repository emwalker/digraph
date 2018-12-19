package loaders

import (
	"database/sql"
)

type config struct {
	db *sql.DB
}

func convertIds(ids []string) []interface{} {
	var translatedIds []interface{}
	for _, id := range ids {
		translatedIds = append(translatedIds, id)
	}
	return translatedIds
}
