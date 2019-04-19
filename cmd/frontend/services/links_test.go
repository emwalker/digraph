package services_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
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
		{
			name:          "A New York Times article",
			inputURL:      "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html?partner=rss&emc=rss",
			canonicalURL:  "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html",
			expectedError: false,
		},
		{
			name:          "An article from the Independent",
			inputURL:      "https://www.independent.co.uk/news/world/middle-east/saudi-arabia-born-babies-streets-abortion-marriage-wedlock-a8867571.html?utm_source=reddit.com",
			canonicalURL:  "https://www.independent.co.uk/news/world/middle-east/saudi-arabia-born-babies-streets-abortion-marriage-wedlock-a8867571.html",
			expectedError: false,
		},
		{
			name:          "An article from Reuters",
			inputURL:      "https://www.reuters.com/article/france-electricity-solarpower/sunny-spell-boosts-french-solar-generation-to-record-level-idUSL8N21U58M?utm_source=reddit.com",
			canonicalURL:  "https://www.reuters.com/article/france-electricity-solarpower/sunny-spell-boosts-french-solar-generation-to-record-level-idUSL8N21U58M",
			expectedError: false,
		},
		{
			name:          "A Business Insider article",
			inputURL:      "https://www.businessinsider.com/gnss-hacking-spoofing-jamming-russians-screwing-with-gps-2019-4?utm_source=reddit.com",
			canonicalURL:  "https://www.businessinsider.com/gnss-hacking-spoofing-jamming-russians-screwing-with-gps-2019-4",
			expectedError: false,
		},
		{
			name:          "A YouTube video",
			inputURL:      "https://www.youtube.com/watch?v=Wx_2SVm9Jgo&list=PLJ8cMiYb3G5eYGt47YpJcNhILyYLmV-tW&index=3&t=0s",
			canonicalURL:  "https://www.youtube.com/watch?v=Wx_2SVm9Jgo",
			expectedError: false,
		},
		{
			name:          "A BuzzFeed article",
			inputURL:      "https://www.buzzfeed.com/craigsilverman/fever-swamp-election?utm_term=.ug4NRgEQDe#.lszgG6PJZr",
			canonicalURL:  "https://www.buzzfeed.com/craigsilverman/fever-swamp-election",
			expectedError: false,
		},
		{
			name:          "A Gmail link",
			inputURL:      "https://mail.google.com/mail/u/0/#inbox",
			canonicalURL:  "https://mail.google.com/mail/u/0/#inbox",
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
