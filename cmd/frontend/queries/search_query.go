package queries

import (
	"fmt"
	"log"
	"strings"

	"github.com/volatiletech/sqlboiler/types"
)

var (
	stringDelim = "e390c488d729"
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

// PostgresTsQueryInput returns a set of wildcard tokens that can be used in a Postgres full text
// search.
func (q Query) PostgresTsQueryInput() interface{} {
	var tokens []string

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
