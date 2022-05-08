package parser_test

import (
	"testing"

	"github.com/emwalker/digraph/golang/cmd/frontend/queries/parser"
	"github.com/volatiletech/sqlboiler/v4/types"
)

func TestWildcardStringArray(t *testing.T) {
	testData := []struct {
		name   string
		input  string
		output string
	}{
		{
			name:   "A simple case",
			input:  "York New",
			output: `{"%York%","%New%"}`,
		},
		{
			name:   "When there is a comma",
			input:  "York New,",
			output: `{"%York%","%New,%"}`,
		},
		{
			name:   "When there is a {",
			input:  "{York New",
			output: `{"%{York%","%New%"}`,
		},
		{
			name:   "When there is a }",
			input:  "York} New",
			output: `{"%York}%","%New%"}`,
		},
		{
			name:   "When there is a %",
			input:  "York% New",
			output: `{"%York%%","%New%"}`,
		},
		{
			name:   "URL-like strings are normalized",
			input:  "https://www.nytimes.com/page?query=",
			output: `{"%https://www.nytimes.com/page%"}`,
		},
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			s := parser.Parse(&td.input)

			actual, ok := s.WildcardStringArray().(*types.StringArray)
			if !ok {
				t.Fatalf("Expected a StringArray, got: %#v", actual)
			}

			value, err := actual.Value()
			if err != nil {
				t.Fatal(err)
			}

			if value != td.output {
				t.Fatalf("Expected %#v, got %#v", td.output, value)
			}
		})
	}
}
