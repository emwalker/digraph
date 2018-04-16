package api

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/graphql-go/graphql"
	_ "github.com/lib/pq"
	_ "github.com/mattes/migrate"
)

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

var connection Connection

func Init(conn Connection) {
	connection = conn
	connection.Init()
}

func Handle(endpoint string, conn Connection) {
	Init(conn)
	http.Handle(endpoint, handler(Schema))
}
