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
	conn := api.NewConnection(
		&api.Credentials{BearerToken: "1234"},
		"postgres",
		"postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable",
	)
	api.Handle("/graphql", conn)

	go func() {
		log.Fatal(http.ListenAndServe("0.0.0.0:8080", nil))
	}()

	Run(os.Args)
}
