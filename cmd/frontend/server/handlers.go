package server

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/cmd/frontend/loaders"
	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/gorilla/handlers"
	"github.com/rs/cors"
)

const (
	userSessionKey = "userSessionKey"
)

var (
	serverSecret = os.Getenv("DIGRAPH_SERVER_SECRET")
)

func serverSecretOrDefault() string {
	if serverSecret == "" {
		return "keyboard cat"
	}
	return serverSecret
}

func must(err error) {
	if err != nil {
		log.Fatal("there was a problem: ", err)
	}
}

func rejectBasicAuth(next http.Handler, w http.ResponseWriter, r *http.Request) {
	w.Header().Set("WWW-Authenticate", `Basic realm="Digraph"`)
	w.WriteHeader(401)
	w.Write([]byte("Unauthorized.\n"))
}

// https://stackoverflow.com/a/39591234/61048
func (s *Server) withBasicAuth(next http.Handler) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		viewerID, sessionID, ok := r.BasicAuth()

		ctx := r.Context()
		rc := resolvers.NewRequestContext(resolvers.GuestViewer)
		rc.SetServerSecret(serverSecretOrDefault())
		ctx = resolvers.WithRequestContext(ctx, rc)

		if !ok {
			log.Printf("Basic auth not ok, continuing as guest: %s", r.Header.Get("Authorization"))
		} else if viewerID != "" {
			session, err := models.FindSession(ctx, s.db, sessionID)

			switch {
			case err != nil:
				log.Printf("There was a problem looking up session: %s", err)
				rejectBasicAuth(next, w, r)
				break
			case session.UserID != viewerID:
				log.Printf("Viewer %s did not match user id %s on session %s", viewerID, session.UserID, session.ID)
				rejectBasicAuth(next, w, r)
				break
			default:
				viewer, err := session.User().One(ctx, s.db)
				if err != nil {
					log.Printf("There was a problem looking up viewer: %s", err)
					rejectBasicAuth(next, w, r)
				} else {
					log.Printf("Viewer %s added to request context", viewer)
					rc.SetViewer(viewer)
				}
			}
		}

		next.ServeHTTP(w, r.WithContext(ctx))
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
