package resolvers

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNormalizeUrl(t *testing.T) {
	url, err := normalizeUrl("http://some.url.com")
	assert.Nil(t, err)
	assert.Equal(t, url.CanonicalURL, "http://some.url.com")

	url, err = normalizeUrl("https://news.ycombinator.com/item?id=18504300")
	assert.Nil(t, err)
	assert.Equal(t, "https://news.ycombinator.com/item?id=18504300", url.CanonicalURL)
}

func TestSha1Value(t *testing.T) {
	url, err := normalizeUrl("http://some.url.com")
	assert.Nil(t, err)
	assert.Equal(t, "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86", url.Sha1)
}
