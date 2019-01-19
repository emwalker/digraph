package services_test

import (
	"testing"

	"github.com/emwalker/digraph/internal/services"
)

func TestNormalizeURL(t *testing.T) {
	testCases := []struct {
		name          string
		inputURL      string
		canonicalURL  string
		expectedError bool
	}{
		{
			name:          "A basic case",
			inputURL:      "http://some.url.com",
			canonicalURL:  "http://some.url.com",
			expectedError: false,
		},
		{
			name:          "Y Combinator comment section",
			inputURL:      "https://news.ycombinator.com/item?id=18504300",
			canonicalURL:  "https://news.ycombinator.com/item?id=18504300",
			expectedError: false,
		},
		{
			name:          "A bugfix",
			inputURL:      "https://quaderno.io/stripe-vat-subscriptions/",
			canonicalURL:  "https://quaderno.io/stripe-vat-subscriptions/",
			expectedError: false,
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			url, err := services.NormalizeURL(testCase.inputURL)
			if err == nil {
				if url.CanonicalURL != testCase.canonicalURL {
					t.Fatalf("Unexpected url: %s, expected: %s", url.CanonicalURL, testCase.canonicalURL)
				}
			} else if !testCase.expectedError {
				t.Fatal(err)
			}
		})
	}
}

func TestSha1Value(t *testing.T) {
	var url *services.URL
	var err error

	if url, err = services.NormalizeURL("http://some.url.com"); err != nil {
		t.Fatal(err)
	}

	if url.Sha1 != "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86" {
		t.Fatalf("Unexpected SHA1: %s", url.Sha1)
	}
}
