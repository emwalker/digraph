package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

func TestDeleteAccount(t *testing.T) {
	ctx := testContext()

	email := "bozbar@frotz.net"
	name := "Bozbar"
	login := "bozbar"

	c := services.Connection{Exec: testDB, Actor: testViewer}
	result, err := c.CreateGithubSession(
		ctx, name, email, login, "https://some/url",
	)
	defer result.Cleanup()

	count, err := models.Users(qm.Where("primary_email = ?", email)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatalf("Expected there to be a user with the email %s", email)
	}

	_, err = models.DeletedUsers().DeleteAll(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	user := result.User
	m := newMutator(t, user)

	deleteAccountInput := models.DeleteAccountInput{UserID: user.ID}

	payload, err := m.resolver.DeleteAccount(m.ctx, deleteAccountInput)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload.Alerts) < 1 {
		t.Fatal("Expected there to be at least one info alert")
	}

	count, err = models.Repositories(qm.Where("owner_id = ?", user.ID)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatalf("Expected repos associated with user %s to have been deleted", email)
	}

	count, err = models.Organizations(qm.Where("login = ? and system", user.Login)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatalf("Expected organization associated with user %s to have been deleted", email)
	}

	count, err = models.Users(qm.Where("primary_email = ?", email)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatalf("Expected user with email %s to have been deleted", email)
	}

	count, err = models.DeletedUsers(qm.Where("user_id = ?", user.ID)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count < 1 {
		t.Fatal("Expected a row to be added to the deleted users table")
	}
}
