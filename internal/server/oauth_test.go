package server

import (
	"testing"
)

func TestOauthCallbackPath(t *testing.T) {
	if callbackPath(true, "8080", "github") != "http://localhost:8080/auth/github/callback" {
		t.Fatal("Wrong development github oauth callback path")
	}

	if callbackPath(false, "", "github") != "https://digraph.app/auth/github/callback" {
		t.Fatal("Wrong production github oauth callback path")
	}
}
