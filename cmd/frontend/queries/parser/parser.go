package parser

import (
	"strings"

	"github.com/emwalker/digraph/cmd/frontend/util"
)

// Parse parses the input string and returns a query spec that can be used for constructing complex SQL
// queries
func Parse(input *string) *QuerySpec {
	if !util.Present(input) {
		return &QuerySpec{Input: input, Tokens: []string{}}
	}

	tokens := strings.Split(string(*input), " ")
	var stringTokens []string
	var topics []TopicSpec

	for _, token := range tokens {
		if strings.HasPrefix(token, "in:") {
			topics = append(topics, TopicSpec{token[3:]})
		} else {
			stringTokens = append(stringTokens, token)
		}
	}

	return &QuerySpec{
		Input:        input,
		Tokens:       tokens,
		stringTokens: stringTokens,
		Topics:       topics,
	}
}
