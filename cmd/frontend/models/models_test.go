package models

import (
	"encoding/hex"
	"testing"
)

func TestHexID(t *testing.T) {
	encodedID := "fd1626fd659b50a6df4d3d79bdd1e777c9b1156ce827dbd2f59bb7d0e232e6e9"

	binaryID, err := hex.DecodeString(encodedID)
	if err != nil {
		t.Fatal(err)
	}

	session := Session{
		ID:        1234,
		SessionID: binaryID,
		UserID:    "1234",
	}

	actualID := session.HexID()
	if actualID != encodedID {
		t.Fatalf("Expected HexID to return %s, got %s", encodedID, actualID)
	}
}
