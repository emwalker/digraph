package main

import (
	"database/sql"
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"text/template"

	"github.com/99designs/gqlgen/graphql"
	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/go-webpack/webpack"
	"github.com/gorilla/handlers"
	_ "github.com/lib/pq"
	"github.com/rs/cors"
)

const defaultPort = "8080"

type server struct {
	connectionString string
	db               *sql.DB
	devMode          bool
	port             string
	schema           graphql.ExecutableSchema
}

func newServer(port string, devMode bool) *server {
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
		connectionString: connectionString,
		db:               db,
		devMode:          devMode,
		port:             port,
		schema:           schema,
	}
}

func main() {
	devMode := flag.Bool("dev", false, "Development mode")
	webpack.Plugin = "manifest"
	webpack.Init(*devMode)

	s := newServer(getPlaygroundPort(), *devMode)
	s.routes()
	s.run()
}

func failIf(err error) {
	if err != nil {
		log.Fatal("there was a problem: ", err)
	}
}

const homepageTemplate = `<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="Content-Language" content="en">
    <title>Digraph</title>
    {{ asset "main.css" }}
  </head>

  <body>
    <div id="root"></div>
    {{ asset "vendors.js" }}
    {{ asset "main.js" }}
  </body>
</html>`


func (s *server) handleRoot() http.Handler {
	funcMap := map[string]interface{}{"asset": webpack.AssetHelper}
	t := template.New("homepage").Funcs(funcMap)
	template.Must(t.Parse(homepageTemplate))

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		t.Execute(w, nil)
	})
}

func (s *server) handleGraphqlRequest() http.Handler {
	h := cors.Default().Handler(handler.GraphQL(s.schema))
	return handlers.CombinedLoggingHandler(os.Stdout, handlers.CompressHandler(h))
}

func (s *server) handleGraphqlPlayground() http.Handler {
	return handler.Playground("GraphQL playground", "/graphql")
}

func (s *server) handleHealthCheck() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})
}

func (s *server) handleStaticFiles() http.Handler {
	fs := http.FileServer(http.Dir("public/webpack"))
	return http.StripPrefix("/static", fs)
}

func (s *server) routes() {
	http.Handle("/static/", s.handleStaticFiles())
	http.Handle("/graphql", s.handleGraphqlRequest())
	http.Handle("/playground", s.handleGraphqlPlayground())
	http.Handle("/_ah/health", s.handleHealthCheck())
	http.Handle("/", s.handleRoot())
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
