package models

import (
	"encoding/hex"
	"testing"

	"github.com/volatiletech/null/v8"
)

func TestSessionHexID(t *testing.T) {
	encodedID := "fd1626fd659b50a6df4d3d79bdd1e777c9b1156ce827dbd2f59bb7d0e232e6e9"

	binaryID, err := hex.DecodeString(encodedID)
	if err != nil {
		t.Fatal(err)
	}

	session := Session{
		ID:        "1233",
		SessionID: binaryID,
		UserID:    "1234",
	}

	actualID := session.HexID()
	if actualID != encodedID {
		t.Fatalf("Expected HexID to return %s, got %s", encodedID, actualID)
	}
}

func TestUserDisplayName(t *testing.T) {
	testCases := []struct {
		name         string
		user         User
		expectedName string
	}{
		{
			name:         "When the name is set",
			user:         User{Name: "Gnusto", Login: null.StringFrom("gnusto")},
			expectedName: "Gnusto",
		},
		{
			name:         "When only the login is set",
			user:         User{Name: "", Login: null.StringFrom("gnusto")},
			expectedName: "gnusto",
		},
		{
			name:         "When neither the name nor the login are set",
			user:         User{Name: "", Login: null.StringFromPtr(nil)},
			expectedName: "<missing name>",
		},
	}

	for _, td := range testCases {
		t.Run(td.name, func(t *testing.T) {
			name := td.user.DisplayName()
			if name != td.expectedName {
				t.Fatalf("Expected '%s', got '%s'", td.expectedName, name)
			}
		})
	}
}
