package main

import (
	"database/sql"
	"fmt"
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
	db               *sql.DB
	schema           graphql.ExecutableSchema
	port             string
	connectionString string
}

func newServer(port string) *server {
	connectionString := os.Getenv("POSTGRES_CONNECTION")
	if connectionString == "" {
		panic("POSTGRES_CONNECTION not set")
	}

	db, err := sql.Open("postgres", connectionString)
	failIf(err)

	err = db.Ping()
	failIf(err)

	resolver := &resolvers.Resolver{DB: db}
	schema := models.NewExecutableSchema(models.Config{Resolvers: resolver})

	return &server{
		db:               db,
		schema:           schema,
		port:             port,
		connectionString: connectionString,
	}
}

func (s *server) handleGraphqlRequest() http.Handler {
	h := cors.Default().Handler(handler.GraphQL(s.schema))
	return handlers.CombinedLoggingHandler(os.Stdout, handlers.CompressHandler(h))
}

func (s *server) handleGraphqlPlayground() http.Handler {
	return handler.Playground("GraphQL playground", "/graphql")
}

func (s *server) healthCheckHandler() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})
}

func handleRoot() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.NotFound(w, r)
			return
		}
		fmt.Fprint(w, "Welcome to Digraph!")
	})
}

func (s *server) handleRoot() http.Handler {
	return handler.Playground("GraphQL playground", "/graphql")
}

func (s *server) routes() {
	http.Handle("/", s.handleRoot())
	http.Handle("/graphql", s.handleGraphqlRequest())
	// http.Handle("/playground", s.handleGraphqlPlayground())
	http.Handle("/_ah/health", s.healthCheckHandler())
}

func (s *server) run() {
	log.Printf("Connect to http://localhost:%s/playground for the GraphQL playground", s.port)
	log.Printf("Listening on port %s", s.port)
	log.Fatal(http.ListenAndServe(":"+s.port, nil))
}

func getPlaygroundPort() string {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	return port
}

func failIf(err error) {
	if err != nil {
		log.Fatal("there was a problem: ", err)
	}
}

func main() {
	s := newServer(getPlaygroundPort())
	s.routes()
	s.run()
}
