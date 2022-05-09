package main

import (
	"flag"
	"log"
	"os"
	"strconv"

	"github.com/emwalker/digraph/golang/internal/server"
	// Load the PQ drivers
	_ "github.com/lib/pq"
)

const defaultPort = "8080"

func getPort() string {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	return port
}

func getLogLevel(logLevel int) int {
	str := os.Getenv("DIGRAPH_LOG_LEVEL")
	if str != "" {
		log.Print("DIGRAPH_LOG_LEVEL found, overriding the command line flag")
		i, err := strconv.Atoi(str)
		if err != nil {
			log.Printf("Unable to parse DIGRAPH_LOG_LEVEL: %s", err)
			return logLevel
		}
		logLevel = i
	} else {
		log.Print("No DIGRAPH_LOG_LEVEL found, falling back to the command line flag")
	}
	return logLevel
}

func main() {
	devMode := flag.Bool("dev", false, "Development mode")
	logLevel := flag.Int("log", 1, "Print debugging information to the console")

	flag.Parse()

	connectionString := os.Getenv("DIGRAPH_POSTGRES_CONNECTION")
	if connectionString == "" {
		panic("DIGRAPH_POSTGRES_CONNECTION not set")
	}

	redisHost := os.Getenv("DIGRAPH_REDIS_HOST")
	if redisHost == "" {
		redisHost = "localhost:6379"
	}

	port := getPort()
	log.Printf("Backend server listening on port %s", port)

	s := server.New(
		port,
		*devMode,
		os.Getenv("DIGRAPH_BASIC_AUTH_USERNAME"),
		os.Getenv("DIGRAPH_BASIC_AUTH_PASSWORD"),
		redisHost,
		os.Getenv("DIGRAPH_REDIS_PASSWORD"),
		getLogLevel(*logLevel),
		connectionString,
	)

	s.Routes()
	s.Run()
}
