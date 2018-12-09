package pageinfo

import (
	"github.com/badoux/goscraper"
)

type PageInfo struct {
	URL   string
	Title *string
}

type Fetcher interface {
	FetchPage(string) (*PageInfo, error)
}

type HtmlFetcher struct{}

func (f *HtmlFetcher) FetchPage(url string) (*PageInfo, error) {
	pageInfo, err := goscraper.Scrape(url, 5)
	if err != nil {
		return nil, err
	}

	return &PageInfo{
		URL:   url,
		Title: &pageInfo.Preview.Title,
	}, nil
}
