package resolvers

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNormalizeUrl(t *testing.T) {
	url, err := normalizeUrl("http://some.url.com")
	assert.Nil(t, err)

	assert.Equal(t, url.CanonicalURL, "http://some.url.com/")
	assert.Equal(t, url.Sha1, "49a19c25e29d440715906aebc07ffbbecfb6037f")
}
