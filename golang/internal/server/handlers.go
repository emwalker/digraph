package server

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/emwalker/digraph/golang/internal/loaders"
	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/resolvers"
	"github.com/gorilla/handlers"
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
		log.Fatal("server: there was a problem: ", err)
	}
}

func rejectBasicAuth(next http.Handler, w http.ResponseWriter, r *http.Request) {
	w.Header().Set("WWW-Authenticate", `Basic realm="Digraph"`)
	w.WriteHeader(401)
	_, err := w.Write([]byte("Unauthorized.\n"))
	if err != nil {
		log.Fatalf("server: failed to reject basic auth: %s", err)
	}
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
			log.Print("server: basic auth not ok, continuing as guest")
		} else if viewerID != "" {
			session, err := models.FindSession(ctx, s.db, sessionID)

			switch {
			case err != nil:
				log.Printf("server: viewer %s: problem looking up session: %s", viewerID, err)
				rejectBasicAuth(next, w, r)
			case session.UserID != viewerID:
				log.Printf("server: viewer %s: did not match user id %s on session %s", viewerID, session.UserID, session.ID)
				rejectBasicAuth(next, w, r)
			default:
				viewer, err := session.User().One(ctx, s.db)
				if err != nil {
					log.Printf("server: viewer %s: there was a problem looking up viewer: %s", viewerID, err)
					rejectBasicAuth(next, w, r)
				} else {
					log.Printf("server: viewer %s: viewer added to request context", viewer)
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
	srv := handler.NewDefaultServer(s.schema)
	h := handlers.CompressHandler(srv)
	if s.LogLevel > 0 {
		h = handlers.CombinedLoggingHandler(os.Stdout, h)
	}
	return s.withLoaders(h)
}

func (s *Server) handleGraphqlPlayground() http.Handler {
	return playground.Handler("GraphQL playground", "/graphql")
}

func (s *Server) handleHealthCheck() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})
}

func (s *Server) handleMock500() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "server: there was a problem", 500)
	})
}
