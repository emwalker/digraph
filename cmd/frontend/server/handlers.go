package server

import (
	"crypto/subtle"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/cmd/frontend/loaders"
	"github.com/gorilla/handlers"
	"github.com/rs/cors"
)

const (
	userSessionKey = "userSessionKey"
)

func must(err error) {
	if err != nil {
		log.Fatal("there was a problem: ", err)
	}
}

func (s *Server) basicAuthRequired(r *http.Request) bool {
	user, pass, ok := r.BasicAuth()

	authFailed := !ok ||
		subtle.ConstantTimeCompare([]byte(user), []byte(s.BasicAuthUsername)) != 1 ||
		subtle.ConstantTimeCompare([]byte(pass), []byte(s.BasicAuthPassword)) != 1

	if authFailed {
		log.Printf("User '%s' did not succeed in authenticating; be sure to pass the basic auth username and password via a header", user)
	}

	return authFailed
}

// https://stackoverflow.com/a/39591234/61048
func (s *Server) withBasicAuth(next http.Handler) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if s.basicAuthRequired(r) {
			w.Header().Set("WWW-Authenticate", `Basic realm="Digraph"`)
			w.WriteHeader(401)
			w.Write([]byte("Unauthorized.\n"))
			return
		}

		next.ServeHTTP(w, r)
	})
}

// https://github.com/vektah/gqlgen-tutorials/blob/master/dataloader/graph.go
func (s *Server) withLoaders(next http.Handler) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()
		ctx = loaders.AddToContext(ctx, s.db, 1*time.Millisecond)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func (s *Server) handleGraphqlRequest() http.Handler {
	handler := cors.Default().Handler(handler.GraphQL(s.schema))
	handler = handlers.CompressHandler(handler)
	if s.LogLevel > 0 {
		handler = handlers.CombinedLoggingHandler(os.Stdout, handler)
	}
	return s.withLoaders(handler)
}

func (s *Server) handleGraphqlPlayground() http.Handler {
	return handler.Playground("GraphQL playground", "/graphql")
}

func (s *Server) handleHealthCheck() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})
}

func (s *Server) handleMock500() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "There was a problem", 500)
	})
}
