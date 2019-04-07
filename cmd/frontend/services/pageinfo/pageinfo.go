package pageinfo

import (
	"bytes"
	"crypto/sha1"
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/microcosm-cc/bluemonday"
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
	Body  []byte
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
	string, err := traverse(doc)
	if err != nil {
		return "", err
	}

	// Remove html elements from title
	p := bluemonday.StrictPolicy()
	return p.Sanitize(string), nil
}

const browserUserAgent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.86 Safari/537.36"

// FetchPage satisfies the Fetcher interface.
func (f *HTMLFetcher) FetchPage(url string) (*PageInfo, error) {
	log.Println("Attempting to fetch url:", url)

	req, err := http.NewRequest("GET", url, nil)
	req.Header.Set("User-Agent", browserUserAgent)

	resp, err := f.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	buf := new(bytes.Buffer)
	buf.ReadFrom(resp.Body)
	body := buf.Bytes()

	doc, err := html.Parse(bytes.NewReader(body))
	if err != nil {
		return nil, err
	}

	title, err := GetHTMLTitle(doc)
	if err != nil {
		return nil, err
	}

	return &PageInfo{URL: url, Title: &title, Body: body}, nil
}

// MissingTitle returns true if no title was found.
func (p *PageInfo) MissingTitle() bool {
	return p.Title == nil || *p.Title == ""
}

// Sha1String returns a sha1 of the contents of the page that was fetched.
func (p *PageInfo) Sha1String() string {
	return fmt.Sprintf("%x", sha1.Sum(p.Body))
}

// WriteToFile writes the contents of the page that was fetched to the path provided.
func (p *PageInfo) WriteToFile(path string) error {
	log.Println(fmt.Sprintf("Writing %s to %s", p.URL, path))

	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()

	if _, err := f.Write(p.Body); err != nil {
		return err
	}

	if err := f.Close(); err != nil {
		return err
	}

	return nil
}
