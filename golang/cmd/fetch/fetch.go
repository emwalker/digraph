package main

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/emwalker/digraph/golang/cmd/frontend/services/pageinfo"
)

func main() {
	log.Println("Fetching urls")
	fetcher := pageinfo.New(&http.Client{})

	for _, url := range os.Args[1:] {
		info, err := fetcher.FetchPage(url)
		if err != nil {
			log.Fatal(err)
		}

		if info.MissingTitle() {
			log.Println(fmt.Sprintf("%s: no title", url))
		} else {
			log.Println(fmt.Sprintf("%s: %s", url, *info.Title))
		}

		info.WriteToFile(fmt.Sprintf("%s.html", info.Sha1String()))
	}
}
