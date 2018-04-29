package api

import (
	"errors"
	"fmt"
	"strings"

	"github.com/cayleygraph/cayley/quad"
	"github.com/graphql-go/graphql"
	"github.com/graphql-go/relay"
	"golang.org/x/net/context"
)

type Organization struct {
	rdfType    struct{} `quad:"@type > foaf:Organization"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Name       string   `json:"name" quad:"di:name"`
}

type User struct {
	rdfType    struct{} `quad:"@type > foaf:Person"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Name       string   `json:"name" quad:"di:name"`
	Email      string   `json:"email" quad:"di:email"`
}

type Topic struct {
	rdfType     struct{} `quad:"@type > foaf:topic"`
	ID          string   `json:"id" quad:",optional"`
	ResourceID  quad.IRI `json:"@id"`
	Name        string   `json:"name" quad:"di:name"`
	Description *string  `json:"description" quad:"description,optional"`
}

type Resource interface {
	Init()
	IRI() quad.IRI
}

var nodeDefinitions *relay.NodeDefinitions
var OrganizationType *graphql.Object
var UserType *graphql.Object
var QueryType *graphql.Object
var TopicType *graphql.Object
var ResourceIdentifiableInterface *graphql.Interface
var MutationType *graphql.Object

var replacer = strings.NewReplacer("<", "", ">", "")

func isomorphicID(id quad.IRI) string {
	return replacer.Replace(id.Short().String())
}

func resourcePath(id quad.IRI) string {
	return replacer.Replace(id.Full().String())
}

func (o *User) Init() {
	o.ID = isomorphicID(o.ResourceID)
}

func (o *User) IRI() quad.IRI {
	return o.ResourceID
}

func (o *Organization) Init() {
	o.ID = isomorphicID(o.ResourceID)
}

func (o *Organization) IRI() quad.IRI {
	return o.ResourceID
}

func (o *Topic) Init() {
	o.ID = isomorphicID(o.ResourceID)
}

func (o *Topic) IRI() quad.IRI {
	return o.ResourceID
}

func resolveType(p graphql.ResolveTypeParams) *graphql.Object {
	switch p.Value.(type) {
	case *Organization:
		return OrganizationType
	case *User:
		return UserType
	default:
		panic("unknown type")
	}
}

func fetcher(conn Connection) relay.IDFetcherFn {
	return func(id string, info graphql.ResolveInfo, ctx context.Context) (interface{}, error) {
		resolvedID := relay.FromGlobalID(id)

		switch resolvedID.Type {
		case "Organization":
			return conn.GetOrganization(resolvedID.ID)
		case "User":
			return conn.GetUser(resolvedID.ID)
		case "Topic":
			return conn.GetTopic(resolvedID.ID)
		default:
			return nil, errors.New(fmt.Sprintf("unknown node type: %s", resolvedID.Type))
		}
	}
}

func organizationField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type: OrganizationType,

		Args: graphql.FieldConfigArgument{
			"resourceId": &graphql.ArgumentConfig{
				Description: "Organization ID",
				Type:        graphql.NewNonNull(graphql.String),
			},
		},

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return conn.GetOrganization(p.Args["resourceId"].(string))
		},
	}
}

func userField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type: UserType,

		Args: graphql.FieldConfigArgument{
			"id": &graphql.ArgumentConfig{
				Description: "User ID",
				Type:        graphql.NewNonNull(graphql.ID),
			},
		},

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return conn.GetUser(p.Args["id"].(string))
		},
	}
}

func viewerField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type: UserType,

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return conn.Viewer()
		},
	}
}

func topicField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type: TopicType,

		Args: graphql.FieldConfigArgument{
			"resourceId": &graphql.ArgumentConfig{
				Description: "Topic ID",
				Type:        graphql.NewNonNull(graphql.String),
			},
		},

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return conn.GetTopic(p.Args["resourceId"].(string))
		},
	}
}

func resourceIdentifierField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type:        graphql.String,
		Description: "The international resource identifier (IRI).",
		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			if res, ok := p.Source.(Resource); ok {
				return isomorphicID(res.IRI()), nil
			}
			return nil, errors.New("unable to provide IRI")
		},
	}
}

func resourcePathField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type:        graphql.String,
		Description: "The relative path to the resource.",
		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			if res, ok := p.Source.(Resource); ok {
				return resourcePath(res.IRI()), nil
			}
			return nil, errors.New("unable to provide resourcePath")
		},
	}
}

func topicsConnection(conn Connection, typ graphql.Output) *graphql.Field {
	return &graphql.Field{
		Type: typ,
		Args: relay.ConnectionArgs,
		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			args := relay.NewConnectionArguments(p.Args)
			dest := []interface{}{}
			if org, ok := p.Source.(*Organization); ok {
				err := conn.FetchTopics(&dest, org)
				if err != nil {
					return nil, err
				}
			}
			return relay.ConnectionFromArray(dest, args), nil
		},
	}
}

func organizationNameField(conn Connection) *graphql.Field {
	return &graphql.Field{
		Type:        graphql.String,
		Description: "The name of the organization.",
	}
}

func createTopicInput(conn Connection) *graphql.InputObject {
	return graphql.NewInputObject(graphql.InputObjectConfig{
		Name:   "CreateTopicInput",
		Fields: graphql.InputObjectConfigFieldMap{},
	})
}

func createTopicMutation(conn Connection, topicEdgeType graphql.Output) *graphql.Field {
	return relay.MutationWithClientMutationID(relay.MutationConfig{
		Name: "CreateTopic",

		InputFields: graphql.InputObjectConfigFieldMap{
			"organizationResourceId": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"name": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"description": &graphql.InputObjectFieldConfig{
				Type:         graphql.String,
				DefaultValue: nil,
			},
		},

		OutputFields: graphql.Fields{
			"topicEdge": &graphql.Field{
				Type: topicEdgeType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if payload, ok := p.Source.(map[string]interface{}); ok {
						topic, err := conn.GetTopic(payload["topicResourceId"].(string))
						checkErr(err)
						return &relay.Edge{Node: topic}, nil
					}
					return nil, nil
				},
			},
		},

		MutateAndGetPayload: func(inputMap map[string]interface{}, info graphql.ResolveInfo, ctx context.Context) (map[string]interface{}, error) {
			orgIri := inputMap["organizationResourceId"].(string)
			name := inputMap["name"].(string)
			description := maybeString(inputMap["description"])
			topic, err := conn.CreateTopic(orgIri, name, description)
			checkErr(err)

			return map[string]interface{}{
				"topicResourceId": topic.ID,
			}, nil
		},
	})
}

func newSchema(conn Connection) (*graphql.Schema, error) {
	nodeDefinitions = relay.NewNodeDefinitions(relay.NodeDefinitionsConfig{
		IDFetcher:   fetcher(conn),
		TypeResolve: resolveType,
	})

	ResourceIdentifiableInterface = graphql.NewInterface(graphql.InterfaceConfig{
		Name: "ResourceIdentifiable",
		Fields: graphql.Fields{
			"resourceId": &graphql.Field{
				Type: graphql.String,
			},
			"resourcePath": &graphql.Field{
				Type: graphql.String,
			},
		},
	})

	UserType = graphql.NewObject(graphql.ObjectConfig{
		Name: "User",
		Fields: graphql.Fields{
			"id": relay.GlobalIDField("User", nil),
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "The name of the user.",
			},
			"email": &graphql.Field{
				Type:        graphql.String,
				Description: "The user's email address.",
			},
		},
		Interfaces: []*graphql.Interface{
			nodeDefinitions.NodeInterface,
		},
	})

	TopicType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Topic",
		Fields: graphql.Fields{
			"id": relay.GlobalIDField("Topic", nil),
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "The name of the topic.",
			},
			"description": &graphql.Field{
				Type:        graphql.String,
				Description: "The description of the topic.",
			},
			"resourceId":   resourceIdentifierField(conn),
			"resourcePath": resourcePathField(conn),
		},
		Interfaces: []*graphql.Interface{
			nodeDefinitions.NodeInterface,
			ResourceIdentifiableInterface,
		},
	})

	topicConnectionDefinition := relay.ConnectionDefinitions(relay.ConnectionConfig{
		Name:     "Topic",
		NodeType: TopicType,
	})

	OrganizationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Organization",
		Fields: graphql.Fields{
			"id":           relay.GlobalIDField("Organization", nil),
			"name":         organizationNameField(conn),
			"topic":        topicField(conn),
			"topics":       topicsConnection(conn, topicConnectionDefinition.ConnectionType),
			"resourceId":   resourceIdentifierField(conn),
			"resourcePath": resourcePathField(conn),
		},
		Interfaces: []*graphql.Interface{
			nodeDefinitions.NodeInterface,
			ResourceIdentifiableInterface,
		},
	})

	QueryType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"viewer":       viewerField(conn),
			"organization": organizationField(conn),
			"user":         userField(conn),
			"node":         nodeDefinitions.NodeField,
		},
	})

	MutationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"createTopic": createTopicMutation(conn, topicConnectionDefinition.EdgeType),
		},
	})

	schema, err := graphql.NewSchema(graphql.SchemaConfig{
		Query:    QueryType,
		Mutation: MutationType,
	})

	return &schema, err
}
