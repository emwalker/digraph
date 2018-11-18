package main

import (
	"database/sql"
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql"
	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/gorilla/handlers"
	_ "github.com/lib/pq"
	"github.com/rs/cors"
)

const defaultPort = "8080"

type server struct {
	db             *sql.DB
	schema         graphql.ExecutableSchema
	playgroundPort string
}

func newServer(playgroundPort string) *server {
	db, err := sql.Open("postgres", "dbname=digraph_dev user=postgres sslmode=disable")
	errIf(err)

	resolver := &resolvers.Resolver{DB: db}
	schema := models.NewExecutableSchema(models.Config{Resolvers: resolver})

	return &server{
		db:             db,
		schema:         schema,
		playgroundPort: playgroundPort,
	}
}

func (s *server) handleGraphqlRequest() http.HandlerFunc {
	return cors.Default().Handler(handler.GraphQL(s.schema)).(http.HandlerFunc)
}

func (s *server) handleGraphqlPlayground() http.HandlerFunc {
	return handler.Playground("GraphQL playground", "/graphql")
}

func (s *server) routes() {
	http.Handle("/graphql", handlers.CombinedLoggingHandler(os.Stdout, http.HandlerFunc(s.handleGraphqlRequest())))
	http.Handle("/", s.handleGraphqlPlayground())
}

func (s *server) run() {
	log.Printf("connect to http://localhost:%s/ for GraphQL playground", s.playgroundPort)
	log.Fatal(http.ListenAndServe(":"+s.playgroundPort, nil))
}

func getPlaygroundPort() string {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	return port
}

func errIf(err error) {
	if err != nil {
		log.Fatalf("there was a problem: %p", err)
	}
}

func main() {
	s := newServer(getPlaygroundPort())
	s.routes()
	s.run()
}
