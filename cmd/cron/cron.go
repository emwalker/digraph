package main

import (
	"database/sql"
	"log"
	"os"

	// Load the PQ drivers
	_ "github.com/lib/pq"
)

func takeSnapshot() {
	log.Println("Taking a snapshot of current metrics")

	connectionString := os.Getenv("DIGRAPH_POSTGRES_CONNECTION")
	if connectionString == "" {
		log.Println("The environment variable DIGRAPH_POSTGRES_CONNECTION must be defined")
		return
	}

	db, err := sql.Open("postgres", connectionString)
	if err != nil {
		log.Printf("Unable to open a connection to postgres: %s", err)
		return
	}
	defer db.Close()

	_, err = db.Exec(`
	with

	topic_stats as (
		select count(*) count from topics
	),

	link_stats as (
		select count(*) count from links
	),

	user_stats as (
		select count(*) count from users
	)

	insert into daily_snapshot (topic_count, link_count, user_count)
		select sum(t.count), sum(l.count), sum(u.count)
		from topic_stats t
		cross join link_stats l
		cross join user_stats u
	`)
	if err != nil {
		log.Printf("Failed to create a new snapshot: %s", err)
		return
	}

	log.Println("Daily snapshot taken")
}

func main() {
	takeSnapshot()
}
