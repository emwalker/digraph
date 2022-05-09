package server

import (
	"context"
	"database/sql"
	"log"
	"net"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/graphql"
	"github.com/NYTimes/gziphandler"
	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/redis"
	"github.com/emwalker/digraph/golang/internal/resolvers"
	"github.com/emwalker/digraph/golang/internal/services/pageinfo"
	"github.com/gorilla/handlers"
	"github.com/volatiletech/sqlboiler/v4/boil"
)

const requestTimeout = 60

// Server holds config information for running the API server.
type Server struct {
	BasicAuthUsername string
	BasicAuthPassword string
	ConnectionString  string
	db                *sql.DB
	DevMode           bool
	LogLevel          int
	Port              string
	rd                redis.Connection
	resolver          *resolvers.Resolver
	schema            graphql.ExecutableSchema
	server            *http.Server
}

// New returns a new *Server configured with the parameters passed in.
func New(
	port string, devMode bool, username, password, redisHost, redisPassword string, logLevel int,
	connectionString string,
) *Server {
	db, err := sql.Open("postgres", connectionString)
	must(err)

	// https://www.alexedwards.net/blog/configuring-sqldb
	db.SetMaxOpenConns(25)
	db.SetMaxIdleConns(25)
	db.SetConnMaxLifetime(5 * time.Minute)

	client := &http.Client{
		Transport: &http.Transport{
			DialContext: (&net.Dialer{
				Timeout: 40 * time.Second,
			}).DialContext,
			TLSHandshakeTimeout: 5 * time.Second,
		},
	}

	fetcher := pageinfo.New(client)

	conn := redis.New(&redis.Options{
		Addr:     redisHost,
		Password: redisPassword,
		DB:       0,
	})

	resolver := resolvers.New(db, fetcher, conn)
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
		rd:                conn,
		schema:            schema,
		server:            server,
	}
}

// https://forum.golangbridge.org/t/how-to-create-custom-timeout-handler-based-on-request-path/7135/2
func logAndTimeout(h http.Handler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ua := r.Header.Get("User-Agent")
		log.Printf("User-Agent: %s", ua)
		timeoutHandler := http.TimeoutHandler(h, requestTimeout*time.Second, "Your request has timed out.")
		loggingHandler := handlers.LoggingHandler(os.Stdout, timeoutHandler)
		loggingHandler.ServeHTTP(w, r)
	}
}

func defaultHandling(h http.Handler) http.Handler {
	withoutGz := logAndTimeout(h)
	return gziphandler.GzipHandler(withoutGz)
}

// Routes registers route handlers with the http server.
func (s *Server) Routes() {
	http.Handle("/graphql", defaultHandling(s.withBasicAuth(s.handleGraphqlRequest())))
	http.Handle("/playground", defaultHandling(s.withBasicAuth(s.handleGraphqlPlayground())))
	http.Handle("/_ah/health", defaultHandling(s.handleHealthCheck()))
	http.Handle("/500", defaultHandling(s.handleMock500()))
}

// Run starts up the http server.
func (s *Server) Run() {
	log.Printf("Running server with log level %d", s.LogLevel)
	if s.LogLevel > 1 {
		boil.DebugMode = true
	}

	err := s.db.Ping()
	if err != nil {
		log.Printf("Postgres is not available")
		panic(err)
	}
	log.Printf("Postgres is available")

	_, err = s.rd.Ping(context.Background()).Result()
	if err != nil {
		log.Printf("Redis is not available")
		panic(err)
	}
	log.Printf("Redis is available")

	log.Printf("Connect to http://localhost:%s/playground for the GraphQL playground", s.Port)
	log.Printf("Listening on port %s", s.Port)
	log.Fatal(s.server.ListenAndServe())
}
