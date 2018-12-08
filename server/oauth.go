package server

import (
	"fmt"
	"html/template"
	"net/http"
	"os"

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
		),
	)

	gothic.GetProviderName = func(r *http.Request) (string, error) {
		return "github", nil
	}

	http.Handle(oauthCallbackRoute("github"))
	http.Handle(oauthLogoutRoute("github"))
	http.Handle(oauthAuthRoute("github"))
}

func callbackPath(devMode bool, port, provider string) string {
	if devMode {
		return fmt.Sprintf("http://localhost:%s/auth/%s/callback", port, provider)
	}
	return fmt.Sprintf("https://digraph.app/auth/%s/callback", provider)
}

func oauthCallbackRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/auth/%s/callback", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		user, err := gothic.CompleteUserAuth(w, r)
		if err != nil {
			fmt.Fprintln(w, err)
			return
		}
		t, _ := template.New("foo").Parse(userTemplate)
		t.Execute(w, user)
	})
}

func oauthLogoutRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/logout/%s", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gothic.Logout(w, r)
		w.Header().Set("Location", "/")
		w.WriteHeader(http.StatusTemporaryRedirect)
	})
}

func oauthAuthRoute(provider string) (string, http.Handler) {
	path := fmt.Sprintf("/auth/%s", provider)

	return path, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Try to get the user without re-authenticating
		if gothUser, err := gothic.CompleteUserAuth(w, r); err == nil {
			t, _ := template.New("foo").Parse(userTemplate)
			t.Execute(w, gothUser)
		} else {
			gothic.BeginAuthHandler(w, r)
		}
	})
}
