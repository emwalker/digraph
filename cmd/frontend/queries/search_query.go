package queries

import (
	"fmt"
	"strings"

	"github.com/volatiletech/sqlboiler/types"
)

// Query encapsulates a search query.
type Query string

// NewSearchQuery returns a helper for constructing wildcard queries.
func NewSearchQuery(input string) *Query {
	q := Query(input)
	return &q
}

// WildcardStringArray returns an array of wildcard tokens that can be used in a SQL query.
func (q Query) WildcardStringArray() interface{} {
	var tokens []string
	for _, s := range strings.Split(string(q), " ") {
		tokens = append(tokens, fmt.Sprintf("%%%s%%", s))
	}
	return types.Array(tokens)
}

// WildcardStringQuery returns a set of wildcard tokens that can be used in a Postgres full text
// search.
func (q Query) WildcardStringQuery() interface{} {
	var tokens []string
	for _, s := range strings.Split(string(q), " ") {
		if s != "" {
			tokens = append(tokens, fmt.Sprintf("%s:*", s))
		}
	}
	return strings.Join(tokens, " & ")
}
