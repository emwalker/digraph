package services_test

import (
	"testing"

	"github.com/emwalker/digraph/services"
)

func TestNormalizeUrl(t *testing.T) {
	var url URL
	var err error

	if url, err = services.NormalizeUrl("http://some.url.com"); err != nil {
		t.Fatal(err)
	}

	if url.CanonicalURL != "http://some.url.com" {
		t.Fatalf("Unexpected url: %s", url.CanonicalURL)
	}

	if url, err = services.NormalizeUrl("https://news.ycombinator.com/item?id=18504300"); err != nil {
		t.Fatal(err)
	}

	if url.CanonicalURL != "https://news.ycombinator.com/item?id=18504300" {
		t.Fatalf("Unexpected url: %s", url.CanonicalURL)
	}
}

func TestSha1Value(t *testing.T) {
	var url URL
	var err error

	if url, err = services.NormalizeUrl("http://some.url.com"); err != nil {
		t.Fatal(err)
	}

	if url.Sha1 != "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86" {
		t.Fatalf("Unexpected SHA1: %s", url.Sha1)
	}
}
