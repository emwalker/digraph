package server

import (
	"database/sql"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/graphql"
	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/gorilla/handlers"
	"github.com/volatiletech/sqlboiler/boil"
)

const requestTimeout = 5

// Server holds config information for running the API server.
type Server struct {
	BasicAuthUsername string
	BasicAuthPassword string
	ConnectionString  string
	db                *sql.DB
	DevMode           bool
	ImpersonateUserID *string
	LogLevel          int
	Port              string
	resolver          *resolvers.Resolver
	schema            graphql.ExecutableSchema
	server            *http.Server
}

// New returns a new *Server configured with the parameters passed in.
func New(
	port string, devMode bool, username, password string, logLevel int, connectionString string,
) *Server {
	db, err := sql.Open("postgres", connectionString)
	must(err)

	resolver := &resolvers.Resolver{DB: db}
	schema := models.NewExecutableSchema(models.Config{Resolvers: resolver})

	server := &http.Server{
		Addr:         ":" + port,
		ReadTimeout:  (requestTimeout + 1) * time.Second,
		WriteTimeout: (requestTimeout + 1) * time.Second,
	}

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
		server:            server,
	}
}

// https://forum.golangbridge.org/t/how-to-create-custom-timeout-handler-based-on-request-path/7135/2
func logAndTimeout(h http.Handler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		timeoutHandler := http.TimeoutHandler(h, requestTimeout*time.Second, "Your request has timed out.")
		loggingHandler := handlers.LoggingHandler(os.Stdout, timeoutHandler)
		loggingHandler.ServeHTTP(w, r)
	}
}

// Routes registers route handlers with the http server.
func (s *Server) Routes() {
	http.Handle("/static/", logAndTimeout(s.withBasicAuth(s.handleStaticFiles())))
	http.Handle("/graphql", logAndTimeout(s.withSession(s.withBasicAuth(s.handleGraphqlRequest()))))
	http.Handle("/playground", logAndTimeout(s.withBasicAuth(s.handleGraphqlPlayground())))
	http.Handle("/_ah/health", logAndTimeout(s.handleHealthCheck()))
	http.Handle("/500", logAndTimeout(s.handleMock500()))
	http.Handle("/", logAndTimeout(http.HandlerFunc(s.withBasicAuth(s.handleRoot()))))
	s.RegisterOauth2Routes()
}

// Run starts up the http server.
func (s *Server) Run() {
	must(s.db.Ping())

	log.Printf("Running server with log level %d", s.LogLevel)
	if s.LogLevel > 1 {
		boil.DebugMode = true
	}

	log.Printf("Connect to http://localhost:%s/playground for the GraphQL playground", s.Port)
	log.Printf("Listening on port %s", s.Port)
	log.Fatal(s.server.ListenAndServe())
}
