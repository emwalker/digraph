package pageinfo

import (
	"crypto/sha1"
	"fmt"
	"log"
	"net/url"
	"strings"

	pl "github.com/PuerkitoBio/purell"
)

// URL holds information about a URL that has been upserted.
type URL struct {
	CanonicalURL string
	Input        string
	Sha1         string
}

const normalizationFlags = pl.FlagRemoveDefaultPort |
	pl.FlagDecodeDWORDHost |
	pl.FlagDecodeOctalHost |
	pl.FlagDecodeHexHost |
	pl.FlagRemoveUnnecessaryHostDots |
	pl.FlagRemoveDotSegments |
	pl.FlagRemoveDuplicateSlashes |
	pl.FlagUppercaseEscapes |
	pl.FlagDecodeUnnecessaryEscapes |
	pl.FlagEncodeNecessaryEscapes |
	pl.FlagSortQuery

var (
	omitQuerySites = []string{
		"amazon.com",
		"theatlantic.com",
		"businessinsider.com",
		"dictionary.com",
		"independent.co.uk",
		"motherjones.com",
		"newyorker.com",
		"nymag.com",
		"nytimes.com",
		"reuters.com",
		"scientificamerican.com",
		"thedailybeast.com",
		"theguardian.com",
		"thehill.com",
		"twitter.com",
	}

	omitFields = []string{
		"fbclid",
		"mbid",
		"rss",
		"via",
	}
)

// IsURL returns true if a string parses as a URL and false otherwise.
func IsURL(str string) bool {
	_, err := url.ParseRequestURI(str)
	if err != nil {
		return false
	}
	return true
}

func removeQueryAndAnchor(parsed *url.URL) bool {
	for _, host := range omitQuerySites {
		if strings.HasSuffix(parsed.Host, host) {
			return true
		}
	}
	return false
}

func stripFragment(parsed *url.URL) bool {
	return !strings.HasSuffix(parsed.Host, "mail.google.com")
}

func removeQueryParam(field string) bool {
	if strings.HasPrefix(field, "utm") {
		return true
	}

	for _, omittedField := range omitFields {
		if omittedField == field {
			return true
		}
	}

	return false
}

// NewURL returns a URL with a canonicalized form and a SHA1.
func NewURL(providedURL string) (*URL, error) {
	value, err := NormalizeURL(providedURL)
	if err != nil {
		log.Printf("Unable to normalize url: %s", err)
		return nil, err
	}

	if !IsURL(value.CanonicalURL) {
		return nil, fmt.Errorf("not a valid link: %s", providedURL)
	}

	return value, nil
}

// NormalizeURL normalizes a url before it is stored in the database.
func NormalizeURL(rawURL string) (*URL, error) {
	parsed, err := url.Parse(rawURL)
	if err != nil {
		return nil, err
	}

	if removeQueryAndAnchor(parsed) {
		parsed.RawQuery = ""
		rawURL = parsed.String()
	} else if strings.HasSuffix(parsed.Host, "youtube.com") {
		query := parsed.Query()

		for key := range query {
			if key == "v" {
				continue
			}
			query.Del(key)
		}

		parsed.RawQuery = query.Encode()
		rawURL = parsed.String()
	} else {
		query := parsed.Query()

		for field := range query {
			if removeQueryParam(field) {
				query.Del(field)
			}
		}

		parsed.RawQuery = query.Encode()
		rawURL = parsed.String()
	}

	flags := normalizationFlags
	if stripFragment(parsed) {
		flags |= pl.FlagRemoveFragment
	}

	canonical, err := pl.NormalizeURLString(rawURL, flags)
	if err != nil {
		return nil, err
	}

	sha1 := fmt.Sprintf("%x", sha1.Sum([]byte(canonical)))
	return &URL{canonical, rawURL, sha1}, nil
}
