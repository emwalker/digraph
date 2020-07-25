package parser

import (
	"reflect"
	"testing"
)

func ptr(str string) *string {
	return &str
}

func TestParsing(t *testing.T) {
	testData := []struct {
		name         string
		input        string
		stringTokens []string
		topics       []TopicSpec
	}{
		{
			name:         "Two tokens",
			input:        "York New",
			stringTokens: []string{"York", "New"},
			topics:       []TopicSpec(nil),
		},
		{
			name:         "An empty string",
			input:        "",
			stringTokens: []string(nil),
			topics:       []TopicSpec(nil),
		},
		{
			name:         "A topic",
			input:        "in:/wiki/topics/96a68720-1415-4e29-8c91-c9a65c516a05",
			stringTokens: []string(nil),
			topics:       []TopicSpec{{resourcePath: "/wiki/topics/96a68720-1415-4e29-8c91-c9a65c516a05"}},
		},
		{
			name:         "An incorrectly-specified topic",
			input:        "in:/wiki/topics/96a68720-1415-4e29-8c91-c9a65c516a05https://www.nytimes.com/",
			stringTokens: []string{"in:/wiki/topics/96a68720-1415-4e29-8c91-c9a65c516a05https://www.nytimes.com/"},
			topics:       []TopicSpec(nil),
		},
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			s := Parse(&td.input)

			if !reflect.DeepEqual(s.StringTokens, td.stringTokens) {
				t.Fatalf("Expected %#v, got %#v", td.stringTokens, s.StringTokens)
			}

			if !reflect.DeepEqual(s.Topics, td.topics) {
				t.Fatalf("Expected %#v, got %#v", td.topics, s.Topics)
			}
		})
	}
}
