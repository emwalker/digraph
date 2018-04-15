package main

import (
	"database/sql"
	"log"
	"os"

	"github.com/codegangsta/cli"
	_ "github.com/lib/pq"
	_ "github.com/mattes/migrate"
)

func main() {
	// defer profile.Start(profile.CPUProfile, profile.ProfilePath(".")).Stop()
	Run(os.Args)
}

// Run creates, configures and runs
// main cli.App
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

var db *sql.DB

func RunServer(c *cli.Context) {
        _, err := sql.Open("postgres", "postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable")
        if err != nil {
                log.Fatal(err)
        }
	app := NewApp(AppOptions{
	// see server/app.go:150
	})
	app.Run()
}
