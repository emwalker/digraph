package api

import (
	"context"
	"errors"
	"fmt"
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
	address     string
	context     context.Context
	driverName  string
	schema      *schema.Config
	session     SessionStore
	store       *graph.Handle
	titleForUrl TitleFetcher
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

func (conn *CayleyConnection) Do(callback func(*graph.Transaction)) error {
	tx := cayley.NewTransaction()
	callback(tx)
	return conn.store.ApplyTransaction(tx)
}

func addParentTopics(tx *graph.Transaction, orgId quad.IRI, node Resource) {
	topicIds := node.ParentTopicIDs()
	if len(topicIds) == 0 {
		tx.AddQuad(
			quad.Make(quad.IRI("topic:root"), quad.IRI("di:includes"), node.IRI(), orgId),
		)
	} else {
		for _, topicId := range topicIds {
			if topicId == "" {
				panic("A topic id cannot be empty")
			}
			tx.AddQuad(
				quad.Make(topicId, quad.IRI("di:includes"), node.IRI(), orgId),
			)
		}
	}
}

func (conn *CayleyConnection) CreateLink(orgId quad.IRI, node *Link) error {
	return conn.Do(func(tx *graph.Transaction) {
		tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("rdf:type"), quad.IRI("di:link"), orgId))
		tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("di:url"), node.URL, orgId))
		tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("di:title"), node.Title, orgId))
		addParentTopics(tx, orgId, node)
	})
}

func (conn *CayleyConnection) CreateTopic(orgId quad.IRI, node *Topic) error {
	return conn.Do(func(tx *graph.Transaction) {
		tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("rdf:type"), quad.IRI("foaf:topic"), orgId))
		tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("di:name"), node.Name, orgId))
		if node.Description != nil {
			tx.AddQuad(quad.Make(node.ResourceID, quad.IRI("di:description"), *node.Description, orgId))
		}
		addParentTopics(tx, orgId, node)
	})
}

func (conn *CayleyConnection) FetchLink(orgId quad.IRI, id string) (interface{}, error) {
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

func (conn *CayleyConnection) FetchTitle(url string) (string, error) {
	return conn.titleForUrl(url)
}

func (conn *CayleyConnection) FetchTopic(orgId quad.IRI, id string) (interface{}, error) {
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

func (conn *CayleyConnection) SelectTopic(orgId quad.IRI, userId string, topicId string) (*Topic, error) {
	checkErr(conn.session.Set(userId, "selectedTopicId", topicId))

	if topicId != "" {
		node, err := conn.FetchTopic(orgId, topicId)
		checkErr(err)
		if node != nil {
			return node.(*Topic), nil
		}
		return nil, errors.New("topic not found")
	}

	return nil, nil
}

func (conn *CayleyConnection) SelectedTopic(orgId quad.IRI, userId string) (*Topic, error) {
	topicId, err := conn.session.Get(userId, "selectedTopicId")
	checkErr(err)

	if topicId != "" {
		node, err := conn.FetchTopic(orgId, topicId)
		if err != nil {
			return nil, err
		}
		return node.(*Topic), nil
	}

	return nil, nil
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

func (conn *CayleyConnection) FetchChildTopicsForTopic(orgId quad.IRI, topicId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store, topicId).
		LabelContext(orgId).
		Out(quad.IRI("di:includes")).
		Has(quad.IRI("rdf:type"), quad.IRI("foaf:topic"))
	return conn.loadIteratorTo(out, path, topicArrayType)
}

func (conn *CayleyConnection) FetchLinks(orgId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store).
		LabelContext(orgId).
		Has(quad.IRI("rdf:type"), quad.IRI("di:link"))
	return conn.loadIteratorTo(out, path, linkArrayType)
}

func (conn *CayleyConnection) FetchParentTopicsForTopic(orgId quad.IRI, topicId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store, topicId).
		LabelContext(orgId).
		In(quad.IRI("di:includes")).
		Has(quad.IRI("rdf:type"), quad.IRI("foaf:topic"))
	return conn.loadIteratorTo(out, path, topicArrayType)
}

func (conn *CayleyConnection) FetchTopics(orgId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store).
		LabelContext(orgId).
		Has(quad.IRI("rdf:type"), quad.IRI("foaf:topic"))
	return conn.loadIteratorTo(out, path, topicArrayType)
}

func (conn *CayleyConnection) FetchTopicsForLink(orgId quad.IRI, linkId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store, linkId).
		LabelContext(orgId).
		In(quad.IRI("di:includes")).
		Has(quad.IRI("rdf:type"), quad.IRI("foaf:topic"))
	return conn.loadIteratorTo(out, path, topicArrayType)
}

func (conn *CayleyConnection) FetchLinksForTopic(orgId quad.IRI, topicId quad.IRI, out *[]interface{}) error {
	path := cayley.StartPath(conn.store, topicId).
		LabelContext(orgId).
		Out(quad.IRI("di:includes")).
		Has(quad.IRI("rdf:type"), quad.IRI("di:link"))
	return conn.loadIteratorTo(out, path, linkArrayType)
}
