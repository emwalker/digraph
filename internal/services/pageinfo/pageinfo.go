package pageinfo

import (
	"github.com/badoux/goscraper"
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
	pageInfo, err := goscraper.Scrape(url, 5)
	if err != nil {
		return nil, err
	}

	return &PageInfo{
		URL:   url,
		Title: &pageInfo.Preview.Title,
	}, nil
}
