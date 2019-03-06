package main

import (
	"database/sql"
	"log"
	"os"

	// Load the PQ drivers
	_ "github.com/lib/pq"
	"github.com/jasonlvhit/gocron"
	// "github.com/volatiletech/sqlboiler/queries"
)

func collectMetrics() {
	log.Println("Taking a snapshot of current metrics")

	connectionString := os.Getenv("DIGRAPH_POSTGRES_CONNECTION")
	if connectionString == "" {
		log.Println("The environment variable DIGRAPH_POSTGRES_CONNECTION must be defined")
		return
	}

	_, err := sql.Open("postgres", connectionString)
	if err != nil {
		log.Println("Unable to open a connection to postgres: %s", err)
		return
	}

	// err = queries.Raw(`
	// select t.* from topics t
	// limit $2
	// `, view.ViewerID, limitFrom(first)).Bind(ctx, exec, &topics)

	log.Println("Snapshot of metrics taken")
}

func main() {
	s := gocron.NewScheduler()
	s.Every(2).Seconds().Do(collectMetrics)
	<- s.Start()
}
