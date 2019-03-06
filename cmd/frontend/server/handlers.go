package server

import (
	"context"
	"crypto/subtle"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"text/template"
	"time"

	"github.com/99designs/gqlgen/handler"
	"github.com/emwalker/digraph/cmd/frontend/loaders"
	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers"
	"github.com/go-webpack/webpack"
	"github.com/gorilla/handlers"
	"github.com/markbates/goth/gothic"
	"github.com/rs/cors"
	"github.com/volatiletech/sqlboiler/queries/qm"
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
	if s.BasicAuthUsername == "" && s.BasicAuthPassword == "" {
		return false
	}

	user, pass, ok := r.BasicAuth()
	return !ok ||
		subtle.ConstantTimeCompare([]byte(user), []byte(s.BasicAuthUsername)) != 1 ||
		subtle.ConstantTimeCompare([]byte(pass), []byte(s.BasicAuthPassword)) != 1
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

func (s *Server) withSession(next http.Handler) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		s.resolver.Actor = &resolvers.GuestUser

		sessionID, err := gothic.GetFromSession(userSessionKey, r)
		if err != nil {
			log.Printf("No user session found: %s", err)
			next.ServeHTTP(w, r)
			return
		}

		ctx := r.Context()

		log.Printf("A session id found, looking up session: %s", sessionID)
		session, err := models.Sessions(
			qm.Load("User"),
			qm.Where("session_id = decode(?, 'hex')", sessionID),
		).One(ctx, s.db)

		if err != nil {
			log.Printf("Session not found for session id %s", sessionID)
			next.ServeHTTP(w, r)
			return
		}

		// Figure out a way to avoid mutating the resolver after the fact
		var actor *models.User
		if s.ImpersonateUserID == nil {
			actor = session.R.User
		} else {
			actor, err = models.Users(qm.Where("id = ?", s.ImpersonateUserID)).One(ctx, s.db)
			if err != nil {
				panic(err)
			}
			log.Printf("Impersonating %s", actor.Summary())
		}
		s.resolver.Actor = actor

		log.Printf("Adding %s to context", actor.Summary())
		ctx = context.WithValue(ctx, resolvers.CurrentUserKey, actor)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func parseTemplate(name, path string) *template.Template {
	funcMap := map[string]interface{}{"asset": webpack.AssetHelper}
	io, err := ioutil.ReadFile(path)
	if err != nil {
		panic(err)
	}
	t := template.New(name).Funcs(funcMap)
	template.Must(t.Parse(string(io)))
	return t
}

func (s *Server) handleRoot() http.Handler {
	variables := struct {
		GAID string
	}{
		GAID: os.Getenv("DIGRAPH_GOOGLE_ANALYTICS_ID"),
	}

	appTemplate := parseTemplate("appTemplate", "public/webpack/layout.html")
	aboutPageTemplate := parseTemplate("aboutPageTemplate", "public/webpack/about.html")

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path[1:] {
		case "robots.txt":
			http.ServeFile(w, r, "public/webpack/robots.txt")
			return
		case "about":
			aboutPageTemplate.Execute(w, variables)
			return
		default:
			appTemplate.Execute(w, variables)
		}
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

func (s *Server) handleStaticFiles() http.Handler {
	fs := http.FileServer(http.Dir("public/webpack"))
	return http.StripPrefix("/static", fs)
}
