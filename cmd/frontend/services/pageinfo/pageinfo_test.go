package pageinfo_test

import (
	"strings"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"golang.org/x/net/html"
)

func TestTitle(t *testing.T) {
	testCases := []struct {
		name  string
		body  string
		title string
	}{
		{
			name: "Simple case",
			body: `
			<html><head>
				<title>Title 1</title>
			</head></html>`,
			title: "Title 1",
		},
		{
			name: "Multiple titles",
			body: `
			<html>
				<head>
					<title>Title 1</title>
					<title>Title 2</title>
				</head>
			</html>
			`,
			title: "Title 1",
		},
		{
			name: "Skipping empty titles",
			body: `
			<html>
				<head>
					<title></title>
					<title></title>
					<title>Title 3</title>
				</head>
			</html>
			`,
			title: "Title 3",
		},
		{
			name:  "Removing html from title contents",
			body:  "<html><head><title><em>Candida auris</em> in Healthcare Facilities</title></head></html>",
			title: "Candida auris in Healthcare Facilities",
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			doc, err := html.Parse(strings.NewReader(testCase.body))

			title, err := pageinfo.GetHTMLTitle(doc)
			if err != nil {
				t.Fatal(err)
			}

			if title != testCase.title {
				t.Fatalf(`expected title "%s", got "%s"`, testCase.title, title)
			}
		})
	}
}
