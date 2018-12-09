package server

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/emwalker/digraph/services"
	"github.com/markbates/goth"
	"github.com/markbates/goth/gothic"
	"github.com/markbates/goth/providers/github"
)

var indexTemplate = `{{range $key,$value:=.Providers}}
    <p><a href="/auth/{{$value}}">Log in with {{index $.ProvidersMap $value}}</a></p>
{{end}}`

var userTemplate = `
<p><a href="/logout/{{.Provider}}">logout</a></p>
<p>Name: {{.Name}} [{{.LastName}}, {{.FirstName}}]</p>
<p>Email: {{.Email}}</p>
<p>NickName: {{.NickName}}</p>
<p>Location: {{.Location}}</p>
<p>AvatarURL: {{.AvatarURL}} <img src="{{.AvatarURL}}"></p>
<p>Description: {{.Description}}</p>
<p>UserID: {{.UserID}}</p>
<p>AccessToken: {{.AccessToken}}</p>
<p>ExpiresAt: {{.ExpiresAt}}</p>
<p>RefreshToken: {{.RefreshToken}}</p>`

func (s *Server) RegisterOauth2Routes() {
	goth.UseProviders(
		github.New(
			os.Getenv("DIGRAPH_GITHUB_CLIENT_ID"),
			os.Getenv("DIGRAPH_GITHUB_CLIENT_SECRET"),
			callbackPath(s.DevMode, s.Port, "github"),
			// Strictly for deduping with other OAuth providers
			"user:email",
		),
	)

	gothic.GetProviderName = func(r *http.Request) (string, error) {
		return "github", nil
	}

	http.Handle(s.oauthCallbackRoute("github"))
	http.Handle(s.oauthLogoutRoute("github"))
	http.Handle(s.oauthAuthRoute("github"))
}

func redirectTo(w http.ResponseWriter, path string) {
	w.Header().Set("Location", path)
	w.WriteHeader(http.StatusTemporaryRedirect)
}

func callbackPath(devMode bool, port, provider string) string {
	if devMode {
		return fmt.Sprintf("http://localhost:%s/auth/%s/callback", port, provider)
	}
	return fmt.Sprintf("https://digraph.app/auth/%s/callback", provider)
}

func (s *Server) maybeSaveSession(gothUser goth.User, w http.ResponseWriter, r *http.Request) error {
	tx, err := s.db.Begin()

	c := services.Connection{Exec: tx, Actor: nil}

	result, err := c.FetchOrMakeSession(r.Context(), gothUser)
	if err != nil {
		return err
	}

	tx.Commit()

	if err = gothic.StoreInSession(userSessionKey, string(result.Session.SessionID), r, w); err != nil {
		log.Printf("Unable to store session id with session: %s", err)
		return err
	}

	log.Printf("Stored session id in session: %s", result.Session.SessionID)
	return nil
}

func (s *Server) oauthCallbackRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/auth/%s/callback", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gothUser, err := gothic.CompleteUserAuth(w, r)
		if err != nil {
			log.Printf("Failed to complete user auth: %s", err)
			fmt.Fprintln(w, err)
			return
		}

		if err = s.maybeSaveSession(gothUser, w, r); err != nil {
			log.Printf("Failed to store user session data: %s", err)
			fmt.Fprintln(w, err)
			return
		}

		log.Print("Redirecting to homepage")
		redirectTo(w, "/")
	})
}

func (s *Server) oauthLogoutRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/logout/%s", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gothic.Logout(w, r)
		redirectTo(w, "/")
	})
}

func (s *Server) oauthAuthRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/auth/%s", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Try to get the user without re-authenticating
		gothUser, err := gothic.CompleteUserAuth(w, r)
		if err != nil {
			log.Printf("No session data found, initiating oauth with %s", provider)
			gothic.BeginAuthHandler(w, r)
			return
		}

		if err = s.maybeSaveSession(gothUser, w, r); err != nil {
			log.Printf("Failed to store user session data: %s", err)
			fmt.Fprintln(w, err)
			return
		}

		log.Printf("Session data found, redirecting to homepage")
		redirectTo(w, "/")
	})
}
