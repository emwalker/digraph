package resolvers

import (
	"fmt"
	"strings"

	"github.com/volatiletech/sqlboiler/types"
)

type query string

func (q query) wildcardStringArray() interface{} {
	var tokens []string
	for _, s := range strings.Split(string(q), " ") {
		tokens = append(tokens, fmt.Sprintf("%%%s%%", s))
	}
	return types.Array(tokens)
}

func (q query) wildcardStringQuery() interface{} {
	var tokens []string
	for _, s := range strings.Split(string(q), " ") {
		if s != "" {
			tokens = append(tokens, fmt.Sprintf("%s:*", s))
		}
	}
	return strings.Join(tokens, " & ")
}
