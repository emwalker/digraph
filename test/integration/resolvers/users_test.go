package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertUser(t *testing.T) {
	m := newMutator(t, testViewer)
	email := "gnusto@frotz.net"
	name := "Gnusto Frotz"
	login := "gfrotz"

	count, err := models.Users(qm.Where("primary_email = ?", email)).Count(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatalf("Expected there to be no users with the email %s", email)
	}

	input := models.UpsertUserInput{
		GithubAvatarURL: "https://some/url",
		GithubUsername:  login,
		Name:            name,
		PrimaryEmail:    email,
	}

	payload, err := m.resolver.UpsertUser(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if payload.UserEdge == nil || payload.UserEdge.Node == nil {
		t.Fatal("There should be a user edge and a node")
	}

	user := payload.UserEdge.Node
	defer func() error {
		_, err := user.Delete(m.ctx, m.db)
		_, err = models.Organizations(qm.Where("login = ?", login)).DeleteAll(m.ctx, m.db)
		return err
	}()

	if user.Name != name {
		t.Fatalf("Expected name to be %s, was %s instead", name, user.Name)
	}

	count, err = models.Users(qm.Where("primary_email = ?", email)).Count(m.ctx, m.db)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatalf("Expected there to be a new user with the email %s", email)
	}
}
