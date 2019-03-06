package pageinfo

import (
	"errors"
	"log"
	"net"
	"net/http"
	"time"

	"github.com/gocolly/colly"
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
type HTMLFetcher struct{}

// FetchPage satisfies the Fetcher interface.
func (f *HTMLFetcher) FetchPage(url string) (*PageInfo, error) {
	log.Println("Attempting to fetch url:", url)

	c := colly.NewCollector(
		colly.UserAgent(userAgentString),
	)

	c.SetRequestTimeout(40 * time.Second)

	c.WithTransport(&http.Transport{
		DialContext: (&net.Dialer{
			Timeout: 40 * time.Second,
		}).DialContext,
	})

	c.OnRequest(func(r *colly.Request) {
		r.Headers.Set("Accept", "*/*")
	})

	c.OnResponse(func(r *colly.Response) {
		log.Println("Success:", r.Request.URL)
	})

	var err error
	var title string

	c.OnError(func(r *colly.Response, err error) {
		log.Println("Error:", r.Request.URL, err)
		err = errPageFetch
	})

	c.OnHTML("html", func(e *colly.HTMLElement) {
		title = e.ChildText("title")
	})

	c.Visit(url)
	c.Wait()

	if err != nil {
		return nil, err
	}

	return &PageInfo{URL: url, Title: &title}, nil
}
