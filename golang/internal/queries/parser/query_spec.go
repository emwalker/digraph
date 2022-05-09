package parser

import (
	"fmt"
	"log"
	"math/rand"
	"strings"
	"time"

	"github.com/emwalker/digraph/golang/internal/services/pageinfo"
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
	// NoTopicID is a topic ID that won't match anything
	NoTopicID = "00000000-0000-0000-0000-000000000000"
)

var (
	src = rand.NewSource(time.Now().UnixNano())
)

// TopicSpec provides information about a topic that has been included in a query
type TopicSpec struct {
	resourcePath string
}

// ID returns the uuid for the topic, assuming the resource path is well-formed.  If it is not, return
// a zeroed-out uuid
func (s TopicSpec) ID() string {
	parts := strings.Split(s.resourcePath, "/")
	if len(parts) < 1 {
		return NoTopicID
	}
	return parts[len(parts)-1]
}

// QuerySpec encapsulates a search query.
type QuerySpec struct {
	Input        *string
	Tokens       []string
	StringTokens []string
	Topics       []TopicSpec
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

// TokenInput returns the search string stripped of special search types
func (s QuerySpec) TokenInput() string {
	return strings.Join(s.StringTokens, " ")
}

// WildcardStringArray returns an array of wildcard tokens that can be used in a SQL query. The assumption
// here is that the individual tokens will be passed through appropriate sanitization.  The tokens in the
// array that is returned are not safe to interpolate directly into a SQL query.
func (s QuerySpec) WildcardStringArray() interface{} {
	var tokens []string
	for _, s := range s.StringTokens {
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

// EscapedPostgresTsQueryInput returns a set of wildcard tokens that can be used in a Postgres full text
// search.
func (s QuerySpec) EscapedPostgresTsQueryInput() interface{} {
	var tokens []string
	stringDelim := randSeq(40)

	for _, token := range s.StringTokens {
		if token != "" {
			if strings.Contains(stringDelim, token) {
				log.Printf("Skipping token containing string delimiter: %s", token)
			} else {
				// Since we're inserting this directly into SQL, an unescaped "?" character" will be
				// interpreted as a parameter
				token = strings.Replace(token, "?", "%3F", -1)
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

// ExplicitTopicIds returns the topic ids explicitly specified in the query
func (s QuerySpec) ExplicitTopicIds() []interface{} {
	var ids []interface{}
	for _, topic := range s.Topics {
		ids = append(ids, topic.ID())
	}
	return ids
}
