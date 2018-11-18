package main

import (
	"database/sql"
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	_ "github.com/lib/pq"
	"github.com/rs/cors"
)

const defaultPort = "8080"

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	db, err := sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
	errIf(err)
	resolver := &resolvers.Resolver{DB: db}

	http.Handle("/", handler.Playground("GraphQL playground", "/graphql"))

	schema := models.NewExecutableSchema(models.Config{Resolvers: resolver})
	http.Handle("/graphql", cors.Default().Handler(handler.GraphQL(schema)))

	log.Printf("connect to http://localhost:%s/ for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}

func errIf(err error) {
	if err != nil {
		log.Fatalf("there was a problem: %p", err)
	}
}
