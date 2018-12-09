package server

import (
	"database/sql"
	"log"
	"net/http"

	"github.com/99designs/gqlgen/graphql"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/volatiletech/sqlboiler/boil"
)

type Server struct {
	BasicAuthUsername string
	BasicAuthPassword string
	ConnectionString  string
	db                *sql.DB
	DevMode           bool
	LogLevel          int
	Port              string
	resolver          *resolvers.Resolver
	schema            graphql.ExecutableSchema
}

func New(
	port string, devMode bool, username, password string, logLevel int, connectionString string,
) *Server {
	db, err := sql.Open("postgres", connectionString)
	must(err)
	must(db.Ping())

	resolver := &resolvers.Resolver{DB: db}
	schema := models.NewExecutableSchema(models.Config{Resolvers: resolver})

	return &Server{
		BasicAuthPassword: password,
		BasicAuthUsername: username,
		ConnectionString:  connectionString,
		db:                db,
		DevMode:           devMode,
		LogLevel:          logLevel,
		Port:              port,
		resolver:          resolver,
		schema:            schema,
	}
}

func (s *Server) Routes() {
	http.Handle("/static/", s.withBasicAuth(s.handleStaticFiles()))
	http.Handle("/graphql", s.withSession(s.withBasicAuth(s.handleGraphqlRequest())))
	http.Handle("/playground", s.withBasicAuth(s.handleGraphqlPlayground()))
	http.Handle("/_ah/health", s.handleHealthCheck())
	http.Handle("/", s.withBasicAuth(s.handleRoot()))
	s.RegisterOauth2Routes()
}

func (s *Server) Run() {
	log.Printf("Running server with log level %d", s.LogLevel)
	if s.LogLevel > 1 {
		boil.DebugMode = true
	}

	log.Printf("Connect to http://localhost:%s/playground for the GraphQL playground", s.Port)
	log.Printf("Listening on port %s", s.Port)
	log.Fatal(http.ListenAndServe(":"+s.Port, nil))
}
