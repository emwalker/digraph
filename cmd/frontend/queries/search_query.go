package queries

import (
	"fmt"
	"log"
	"math/rand"
	"strings"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/volatiletech/sqlboiler/v4/types"
)

const (
	letterBytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	// 6 bits to represent a letter index
	letterIdxBits = 6
	// All 1-bits, as many as letterIdxBits
	letterIdxMask = 1<<letterIdxBits - 1
	// # of letter indices fitting in 63 bits
	letterIdxMax = 63 / letterIdxBits
)

var (
	src = rand.NewSource(time.Now().UnixNano())
)

// Query encapsulates a search query.
type Query string

// NewSearchQuery returns a helper for constructing wildcard queries.
func NewSearchQuery(input string) *Query {
	q := Query(input)
	return &q
}

// See https://stackoverflow.com/a/31832326/61048
func randSeq(n int) string {
	sb := strings.Builder{}
	sb.Grow(n)
	// A src.Int63() generates 63 random bits, enough for letterIdxMax characters!
	for i, cache, remain := n-1, src.Int63(), letterIdxMax; i >= 0; {
		if remain == 0 {
			cache, remain = src.Int63(), letterIdxMax
		}
		if idx := int(cache & letterIdxMask); idx < len(letterBytes) {
			sb.WriteByte(letterBytes[idx])
			i--
		}
		cache >>= letterIdxBits
		remain--
	}

	return sb.String()
}

// WildcardStringArray returns an array of wildcard tokens that can be used in a SQL query.
func (q Query) WildcardStringArray() interface{} {
	var tokens []string
	for _, s := range strings.Split(string(q), " ") {
		if pageinfo.IsURL(s) {
			url, err := pageinfo.NormalizeURL(s)
			if err == nil {
				s = url.CanonicalURL
			}
		}
		tokens = append(tokens, fmt.Sprintf("%%%s%%", s))
	}
	return types.Array(tokens)
}

// PostgresTsQueryInput returns a set of wildcard tokens that can be used in a Postgres full text
// search.
func (q Query) PostgresTsQueryInput() interface{} {
	var tokens []string
	stringDelim := randSeq(40)

	for _, token := range strings.Split(string(q), " ") {
		if token != "" {
			if strings.Contains(stringDelim, token) {
				log.Printf("Skipping token containing string delimiter: %s", token)
			} else {
				newToken := fmt.Sprintf("quote_literal($%s$%s$%s$) || ':*'", stringDelim, token, stringDelim)
				tokens = append(tokens, newToken)
			}
		}
	}
	if len(tokens) < 1 {
		return "''"
	}
	return strings.Join(tokens, " || ' & ' || ")
}
