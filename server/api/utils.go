package api

import (
	"context"
	"fmt"
	"reflect"

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

// https://stackoverflow.com/a/34637234/61048
func PointersOf(v interface{}) interface{} {
	in := reflect.ValueOf(v)
	out := reflect.MakeSlice(
		reflect.SliceOf(reflect.PtrTo(in.Type().Elem())),
		in.Len(),
		in.Len(),
	)
	for i := 0; i < in.Len(); i++ {
		out.Index(i).Set(in.Index(i).Addr())
	}
	return out.Interface()
}

func maybeString(str interface{}) *string {
	if value, ok := str.(string); ok {
		return &value
	}
	return nil
}

func stringOr(defaultValue string, maybeString interface{}) string {
	if value, ok := maybeString.(string); ok {
		return value
	}
	return defaultValue
}

func stringList(value interface{}) *[]string {
	if elements, ok := value.([]interface{}); ok {
		out := make([]string, len(elements))
		for i, str := range elements {
			out[i] = str.(string)
		}
		return &out
	}
	return &[]string{}
}
