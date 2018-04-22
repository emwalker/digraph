package api

import (
	"context"
	"fmt"

	"github.com/cayleygraph/cayley/graph"
)

func LastOr(defaultValue string, slice []string) string {
	if len(slice) == 0 {
		return defaultValue
	}
	return slice[len(slice)-1]
}

func ensure(result interface{}, err error) interface{} {
	if err != nil {
		panic(err)
	}
	return result
}

func checkErr(err error) {
	if err != nil {
		panic(err)
	}
}

func dumpStore(store *graph.Handle) {
	fmt.Println("dump of quad store:")
	it := store.QuadsAllIterator()
	ctx := context.TODO()
	for it.Next(ctx) {
		fmt.Println(store.Quad(it.Result()))
	}
}
