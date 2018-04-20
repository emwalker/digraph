package main

import (
	"github.com/emwalker/digraffe/server/api"
)

func main() {
	conn := api.NewConnection(
		"postgres",
		"postgres://postgres@localhost:5432/digraffe_dev?sslmode=disable",
	)

	apiApp, err := api.New(conn)
	if err != nil {
		panic(err)
	}

	go func() {
		apiApp.Run()
	}()

	webApp := NewApp(AppOptions{})
	webApp.Run()
}
