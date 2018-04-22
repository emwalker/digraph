package api

import (
	"fmt"
	"reflect"
	"sort"
	"strings"

	"github.com/cayleygraph/cayley"
	"github.com/cayleygraph/cayley/graph"
	_ "github.com/cayleygraph/cayley/graph/sql/postgres"
	"github.com/cayleygraph/cayley/quad"
	"github.com/cayleygraph/cayley/schema"
	"github.com/cayleygraph/cayley/voc"
	_ "github.com/cayleygraph/cayley/voc/core"
	"github.com/segmentio/ksuid"
)

func init() {
	voc.RegisterPrefix("foaf:", "http://xmlns.com/foaf/spec/")
	voc.RegisterPrefix("di:", "http://github.com/emwalker/digraffe/")
	voc.RegisterPrefix("topic:", "/topics/")
	voc.RegisterPrefix("organization:", "/organizations/")
	voc.RegisterPrefix("user:", "/users/")
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

func (conn *CayleyConnection) GetOrganization(iri string) (interface{}, error) {
	var o Organization
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(iri))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) GetTopic(iri string) (interface{}, error) {
	var o Topic
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(iri))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) GetUser(iri string) (interface{}, error) {
	var o User
	err := conn.schema.LoadTo(nil, conn.store, &o, quad.IRI(iri))
	o.Init()
	return handleResult(&o, err)
}

func (conn *CayleyConnection) Viewer() (interface{}, error) {
	return conn.GetUser("user:gnusto")
}

func (conn *CayleyConnection) SelectOrganizationTopics(
	out *[]interface{},
	organization *Organization,
) error {
	var topics []Topic

	err := schema.Global().LoadTo(nil, conn.store, &topics)
	if err != nil {
		return err
	}

	sort.Slice(topics, func(i, j int) bool {
		return topics[i].Name < topics[j].Name
	})

	for _, topic := range topics {
		topic.Init()
		*out = append(*out, topic)
	}

	return nil
}
