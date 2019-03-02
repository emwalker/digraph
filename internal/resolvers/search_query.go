package resolvers

import (
	"fmt"
	"strings"
)

// SearchQuery holds information on a query to be performed.
type SearchQuery struct {
	Input string
}

// ArrayLikeParameter splits the input string and turns it into a postgres string array
// with wildcards around each token.
func (q SearchQuery) ArrayLikeParameter() string {
	var tokens []string
	for _, s := range strings.Split(q.Input, " ") {
		s = strings.Replace(s, ",", "\\,", -1)
		s = strings.Replace(s, "{", "\\{", -1)
		s = strings.Replace(s, "}", "\\}", -1)
		s = strings.Replace(s, "%", "\\%", -1)
		tokens = append(tokens, fmt.Sprintf("%%%s%%", s))
	}
	return fmt.Sprintf("{%s}", strings.Join(tokens, ","))
}
