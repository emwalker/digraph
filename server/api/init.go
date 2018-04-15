package api

import (
	"database/sql"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"

	"github.com/graphql-go/graphql"
	_ "github.com/lib/pq"
	_ "github.com/mattes/migrate"
)

var db *sql.DB
var schema graphql.Schema

func handler(schema graphql.Schema) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		query, err := ioutil.ReadAll(r.Body)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		result := graphql.Do(graphql.Params{
			Schema:        schema,
			RequestString: string(query),
		})
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(result)
	}
}

func Init() {
	var err error

	schema, err = graphql.NewSchema(graphql.SchemaConfig{
		Query:    QueryType,
		Mutation: MutationType,
	})
	if err != nil {
		log.Fatal(err)
	}

	db, err = sql.Open("postgres", "postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable")
	if err != nil {
		log.Fatal(err)
	}

	http.Handle("/graphql", handler(schema))
}
