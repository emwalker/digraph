package parser

import (
	"regexp"
	"strings"

	"github.com/emwalker/digraph/cmd/frontend/util"
)

var (
	topicPathPattern = "^in:/\\w+/topics/[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}$"
	topicPathRegex   *regexp.Regexp
)

func init() {
	topicPathRegex = regexp.MustCompile(topicPathPattern)
}

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
		if topicPathRegex.MatchString(token) {
			topics = append(topics, TopicSpec{token[3:]})
		} else {
			stringTokens = append(stringTokens, token)
		}
	}

	return &QuerySpec{
		Input:        input,
		Tokens:       tokens,
		StringTokens: stringTokens,
		Topics:       topics,
	}
}
