package main

import (
	"log"
	"net/http"
	"os"

	"github.com/codegangsta/cli"
	"github.com/emwalker/digraffe/server/api"
)

func Run(args []string) {
	app := cli.NewApp()
	app.Name = "app"
	app.Usage = "React server application"

	app.Commands = []cli.Command{
		{
			Name:   "run",
			Usage:  "Runs server",
			Action: RunServer,
		},
	}
	app.Run(args)
}

func RunServer(c *cli.Context) {
	app := NewApp(AppOptions{})
	app.Run()
}

func main() {
	api.Init()

	go func() {
		log.Fatal(http.ListenAndServe("0.0.0.0:8080", nil))
	}()

	Run(os.Args)
}
