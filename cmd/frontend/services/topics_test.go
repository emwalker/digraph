package services_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
)

func TestNormalizeName(t *testing.T) {
	testCases := []struct {
		description    string
		inputName      string
		normalizedName string
		isValid        bool
	}{
		{
			description:    "An valid topic",
			inputName:      "   Agricultural   revolution ",
			normalizedName: "Agricultural revolution",
			isValid:        true,
		},
		{
			description:    "An empty topic",
			inputName:      "   ",
			normalizedName: "",
			isValid:        false,
		},
		{
			description:    "A link",
			inputName:      "http://www.google.com/",
			normalizedName: "http://www.google.com/",
			isValid:        false,
		},
		{
			description:    "An FTP url",
			inputName:      "ftp://ftp.google.com/",
			normalizedName: "ftp://ftp.google.com/",
			isValid:        false,
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.description, func(t *testing.T) {
			normalizedName, ok := services.NormalizeTopicName(testCase.inputName)
			if ok != testCase.isValid {
				t.Fatalf("Expected name to be considered valid:%t, got valid:%t", testCase.isValid, ok)
			}

			if normalizedName != testCase.normalizedName {
				t.Fatalf("Expected normalized name: %s, got: %s", testCase.normalizedName, normalizedName)
			}
		})
	}
}
