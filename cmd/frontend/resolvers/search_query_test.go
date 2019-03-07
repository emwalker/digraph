package resolvers

import (
	"testing"

	"github.com/volatiletech/sqlboiler/types"
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
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			actual, ok := wildcardStringArray(td.input).(*types.StringArray)
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
