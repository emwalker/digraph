package main

import (
	"github.com/emwalker/digraffe/server/api"
)

func main() {
	conn := api.NewConnection(
		"postgres",
		"postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable",
	)

	webApp := NewApp(AppOptions{})

	apiApp, err := api.New(conn, webApp.Engine)
	if err != nil {
		panic(err)
	}

	apiApp.Run()
}
