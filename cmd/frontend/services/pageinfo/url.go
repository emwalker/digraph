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

type urlSpec struct {
	suffix     string
	keepParams []string
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
	stripParams = []string{
		"__twitter_impression",
		"_osource",
		"_returnURL",
		"amp",
		"fbclid",
		"mbid",
		"rss",
		"s_cid",
		"via",
		"redirectedFrom",
	}

	schemes = []string{
		"ftp",
		"git",
		"gopher",
		"http",
		"https",
		"ssh",
	}

	// Applies to any URL that does not have a specific config.
	generalURLSpec = urlSpec{keepParams: []string{"id"}}

	urlSpecs = []urlSpec{
		urlSpec{suffix: "youtube.com", keepParams: []string{"v"}},
		urlSpec{suffix: "urbandictionary.com", keepParams: []string{"term"}},
		urlSpec{suffix: "facebook.com", keepParams: []string{"__xts__[0]", "v"}},
		urlSpec{suffix: "ycombinator.com", keepParams: []string{"id"}},
		urlSpec{suffix: "dur.ac.uk", keepParams: []string{"itemno"}},
		urlSpec{suffix: "baylor.edu", keepParams: []string{"action", "story"}},
		urlSpec{suffix: "umass.edu", keepParams: []string{"article", "context"}},
		urlSpec{suffix: "sdsu.edu", keepParams: []string{"sid"}},
		urlSpec{suffix: "nzherald.co.nz", keepParams: []string{"objectid"}},
		urlSpec{suffix: "amazon.com"},
		urlSpec{suffix: "businessinsider.com"},
		urlSpec{suffix: "dictionary.com"},
		urlSpec{suffix: "huffingtonpost.co.uk"},
		urlSpec{suffix: "independent.co.uk"},
		urlSpec{suffix: "motherjones.com"},
		urlSpec{suffix: "newyorker.com"},
		urlSpec{suffix: "npr.org"},
		urlSpec{suffix: "nymag.com"},
		urlSpec{suffix: "nytimes.com"},
		urlSpec{suffix: "reuters.com"},
		urlSpec{suffix: "scientificamerican.com"},
		urlSpec{suffix: "theatlantic.com"},
		urlSpec{suffix: "thedailybeast.com"},
		urlSpec{suffix: "theguardian.com"},
		urlSpec{suffix: "thehill.com"},
		urlSpec{suffix: "twitter.com"},
	}
)

func (s *urlSpec) matchesHost(host string) bool {
	return strings.HasSuffix(host, s.suffix)
}

func (s *urlSpec) normalizeURL(parsed *url.URL) string {
	query := parsed.Query()

Loop:
	for queryParam := range query {
		for _, keepParam := range s.keepParams {
			if queryParam == keepParam {
				continue Loop
			}
		}
		query.Del(queryParam)
	}

	// FIXME: don't modify the input argument
	parsed.RawQuery = query.Encode()
	return parsed.String()
}

// IsURL returns true if a string parses as a URL and false otherwise.
func IsURL(str string) bool {
	parsed, err := url.ParseRequestURI(str)
	if err != nil {
		return false
	}
	for _, scheme := range schemes {
		if scheme == parsed.Scheme {
			return true
		}
	}
	return false
}

func stripFragment(parsed *url.URL) bool {
	return !strings.HasSuffix(parsed.Host, "mail.google.com")
}

func removeQueryParam(param string) bool {
	if strings.HasPrefix(param, "utm") {
		return true
	}

	for _, omittedParam := range stripParams {
		if omittedParam == param {
			return true
		}
	}

	return false
}

func urlSpecFor(host string) urlSpec {
	for _, spec := range urlSpecs {
		if spec.matchesHost(host) {
			return spec
		}
	}
	return generalURLSpec
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
	copiedURL := rawURL
	parsed, err := url.Parse(copiedURL)
	if err != nil {
		return nil, err
	}

	spec := urlSpecFor(parsed.Host)
	copiedURL = spec.normalizeURL(parsed)

	flags := normalizationFlags
	if stripFragment(parsed) {
		flags |= pl.FlagRemoveFragment
	}

	canonical, err := pl.NormalizeURLString(copiedURL, flags)
	if err != nil {
		return nil, err
	}

	sha1 := fmt.Sprintf("%x", sha1.Sum([]byte(canonical)))
	return &URL{canonical, rawURL, sha1}, nil
}
