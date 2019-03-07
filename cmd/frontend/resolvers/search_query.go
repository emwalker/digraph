package resolvers

import (
	"fmt"
	"strings"

	"github.com/volatiletech/sqlboiler/types"
)

func wildcardStringArray(input string) interface{} {
	var tokens []string
	for _, s := range strings.Split(input, " ") {
		tokens = append(tokens, fmt.Sprintf("%%%s%%", s))
	}
	return types.Array(tokens)
}
