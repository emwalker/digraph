package api

import (
	"context"
	"fmt"
	"log"
	"reflect"
	"sort"
	"strings"

	"github.com/cayleygraph/cayley"
	"github.com/cayleygraph/cayley/graph"
	"github.com/cayleygraph/cayley/graph/path"
	_ "github.com/cayleygraph/cayley/graph/sql/postgres"
	"github.com/cayleygraph/cayley/quad"
	"github.com/cayleygraph/cayley/schema"
	"github.com/cayleygraph/cayley/voc"
	_ "github.com/cayleygraph/cayley/voc/core"
	"github.com/segmentio/ksuid"
)

type Sortable interface {
	Sort()
}

type topicArray []Topic
type linkArray []Link

var topicArrayType reflect.Type
var linkArrayType reflect.Type

func (array topicArray) Sort() {
	sort.Slice(array, func(i, j int) bool {
		return array[i].Name < array[j].Name
	})
}

func (array linkArray) Sort() {
	sort.Slice(array, func(i, j int) bool {
		return array[i].Title < array[j].Title
	})
}

func init() {
	voc.RegisterPrefix("foaf:", "http://xmlns.com/foaf/spec/")
	voc.RegisterPrefix("di:", "http://github.com/emwalker/digraffe/")
	voc.RegisterPrefix("rdf:", "http://www.w3.org/1999/02/22-rdf-syntax-ns")
	voc.RegisterPrefix("topic:", "/topics/")
	voc.RegisterPrefix("link:", "/links/")
	voc.RegisterPrefix("organization:", "/organizations/")
	voc.RegisterPrefix("user:", "/users/")
	topicArrayType = reflect.ValueOf(topicArray{}).Type()
	linkArrayType = reflect.ValueOf(linkArray{}).Type()
}

func generateIDForType(typ string) quad.IRI {
	return quad.IRI(fmt.Sprintf("%s:%s", typ, makeKSUID()))
}

func generateID(o interface{}) quad.Value {
	fullType := reflect.TypeOf(o).String()
	typ := strings.ToLower(LastOr("", strings.Split(fullType, ".")))
	return generateIDForType(typ)
}

func makeKSUID() string {
	return ksuid.New().String()
}

type CayleyConnection struct {
	address    string
	driverName string
	schema     *schema.Config
	store      *graph.Handle
	context    context.Context
}

func (conn *CayleyConnection) Close() error {
	return conn.store.Close()
}

func handleResult(o interface{}, err error) (interface{}, error) {
	if err != nil {
		if err.Error() == "not found" {
			return nil, nil
		}
		return nil, err
	}
	return o, nil
}

func (conn *CayleyConnection) Init() error {
	sch := schema.NewConfig()
	sch.GenerateID = generateID
	conn.schema = sch

	store, err := cayley.NewGraph(conn.driverName, conn.address, nil)
	checkErr(err)
	conn.store = store
	return nil
}

func (conn *CayleyConnection) saveNode(orgId string, resourceId quad.IRI, node interface{}) {
	writer := graph.NewWriter(conn.store)

	_, err := conn.schema.WriteAsQuads(writer, node)
	checkErr(err)
	log.Println("created resource with id", resourceId)

	conn.store.AddQuad(quad.Make(quad.IRI(orgId), quad.IRI("di:owns"), resourceId, nil))
	checkErr(err)

	checkErr(writer.Close())
}

func (conn *CayleyConnection) CreateLink(orgId string, url string, title string) (*Link, error) {
	resourceId := generateIDForType("link")
	node := Link{
		ResourceID: resourceId,
		Title:      title,
		URL:        url,
	}
	node.Init()
	conn.saveNode(orgId, resourceId, node)
	return &node, nil
}

func (conn *CayleyConnection) CreateTopic(orgId string, name string, description *string) (*Topic, error) {
	resourceId := generateIDForType("topic")
	node := Topic{
		Description: description,
		Name:        name,
		ResourceID:  resourceId,
	}
	node.Init()
	conn.saveNode(orgId, resourceId, node)
	return &node, nil
}

func (conn *CayleyConnection) FetchLink(id string) (interface{}, error) {
	var o Link
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(id))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) FetchOrganization(id string) (interface{}, error) {
	var o Organization
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(id))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) FetchTopic(id string) (interface{}, error) {
	var o Topic
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(id))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) FetchUser(id string) (interface{}, error) {
	var o User
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(id))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) Viewer() (interface{}, error) {
	return conn.FetchUser("user:gnusto")
}

func (conn *CayleyConnection) loadIteratorTo(
	out *[]interface{},
	path *path.Path,
	valueType reflect.Type,
) error {
	it, _ := path.BuildIterator().Optimize()
	it, _ = conn.store.OptimizeIterator(it)

	in := reflect.New(valueType)
	err := schema.Global().LoadIteratorTo(conn.context, conn.store, in, it)
	checkErr(err)

	slice := in.Elem()
	if sortable, ok := slice.Interface().(Sortable); ok {
		sortable.Sort()
	}

	for i := 0; i < slice.Len(); i++ {
		ptr := slice.Index(i).Addr().Interface()
		ptr.(Resource).Init()
		*out = append(*out, ptr)
	}

	return nil
}

func (conn *CayleyConnection) FetchTopics(out *[]interface{}, o *Organization) error {
	path := cayley.StartPath(conn.store, o.ResourceID).
		Out(quad.IRI("di:owns")).
		Has(quad.IRI("rdf:type"), quad.IRI("foaf:topic"))
	return conn.loadIteratorTo(out, path, topicArrayType)
}

func (conn *CayleyConnection) FetchLinks(out *[]interface{}, o *Organization) error {
	path := cayley.StartPath(conn.store, o.ResourceID).
		Out(quad.IRI("di:owns")).
		Has(quad.IRI("rdf:type"), quad.IRI("di:link"))
	return conn.loadIteratorTo(out, path, linkArrayType)
}
