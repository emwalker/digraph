package main

import (
	"database/sql"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"os"

	"github.com/codegangsta/cli"
	"github.com/graphql-go/graphql"
	_ "github.com/lib/pq"
	_ "github.com/mattes/migrate"
)

func Run(args []string) {
	app := cli.NewApp()
	app.Name = "app"
	app.Usage = "React server application"

	app.Commands = []cli.Command{
		{
			Name:   "run",
			Usage:  "Runs server",
			Action: RunServer,
		},
	}
	app.Run(args)
}

func RunServer(c *cli.Context) {
	app := NewApp(AppOptions{})
	app.Run()
}

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

var db *sql.DB

func main() {
	schema, err := graphql.NewSchema(graphql.SchemaConfig{
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
	go func() {
		log.Fatal(http.ListenAndServe("0.0.0.0:8080", nil))
	}()

	Run(os.Args)
}
