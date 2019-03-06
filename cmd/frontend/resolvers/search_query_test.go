package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/resolvers"
)

func TestArrayLikeParameter(t *testing.T) {
	testData := []struct {
		name   string
		input  string
		output string
	}{
		{
			name:   "A simple case",
			input:  "York New",
			output: "{%York%,%New%}",
		},
		{
			name:   "When there is a comma",
			input:  "York New,",
			output: "{%York%,%New\\,%}",
		},
		{
			name:   "When there is a {",
			input:  "{York New",
			output: "{%\\{York%,%New%}",
		},
		{
			name:   "When there is a }",
			input:  "York} New",
			output: "{%York\\}%,%New%}",
		},
		{
			name:   "When there is a %",
			input:  "York% New",
			output: "{%York\\%%,%New%}",
		},
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			q := resolvers.SearchQuery{td.input}
			actual := q.ArrayLikeParameter()
			if actual != td.output {
				t.Fatalf("Expected %s, got %s", td.output, actual)
			}
		})
	}
}
