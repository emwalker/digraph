package main

import (
	"log"

	"github.com/badoux/goscraper"
	"github.com/emwalker/digraffe/server/api"
)

func titleFetcher(url string) (string, error) {
	log.Println("fetching page", url, "to get the title ...")
	page, err := goscraper.Scrape(url, 5)
	if err != nil {
		return "", err
	}
	return page.Preview.Title, nil
}

func main() {
	webApp := NewApp(AppOptions{})

	apiApp, err := api.New(&api.Config{
		Address:    "postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable",
		DriverName: "postgres",
		Engine:     webApp.Engine,
		FetchTitle: titleFetcher,
	})
	if err != nil {
		panic(err)
	}

	apiApp.Run()
}
