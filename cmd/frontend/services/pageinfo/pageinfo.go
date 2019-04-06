package pageinfo

import (
	"errors"
	"log"
	"net/http"

	"golang.org/x/net/html"
)

var (
	errPageFetch    = errors.New("unable to fetch page")
	userAgentString = "Digraph Agent"
)

// PageInfo holds information about a page that has been fetched.
type PageInfo struct {
	URL   string
	Title *string
}

// Fetcher is an interface for structs that fetch pages from the internet.
type Fetcher interface {
	FetchPage(string) (*PageInfo, error)
}

// HTMLFetcher is the default fetcher.
type HTMLFetcher struct {
	client *http.Client
}

// New returns an HTMLFetcher initialized with the http.Client passed in.
func New(client *http.Client) *HTMLFetcher {
	return &HTMLFetcher{client}
}

func isTitleElement(n *html.Node) bool {
	return n.Type == html.ElementNode &&
		n.Data == "title" &&
		n.FirstChild != nil
}

func traverse(n *html.Node) (string, error) {
	if isTitleElement(n) {
		return n.FirstChild.Data, nil
	}

	for c := n.FirstChild; c != nil; c = c.NextSibling {
		if c.Type != html.ElementNode {
			continue
		}

		result, err := traverse(c)
		if err != nil {
			return "", err
		}

		if result != "" {
			return result, nil
		}
	}

	return "", nil
}

// GetHTMLTitle extracts the title from a parsed document.
func GetHTMLTitle(doc *html.Node) (string, error) {
	return traverse(doc)
}

// FetchPage satisfies the Fetcher interface.
func (f *HTMLFetcher) FetchPage(url string) (*PageInfo, error) {
	log.Println("Attempting to fetch url:", url)

	resp, err := f.client.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	doc, err := html.Parse(resp.Body)
	if err != nil {
		return nil, err
	}

	title, err := GetHTMLTitle(doc)
	if err != nil {
		return nil, err
	}

	log.Println("Found title:", title)
	return &PageInfo{URL: url, Title: &title}, nil
}
