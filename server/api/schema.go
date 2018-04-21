package api

import (
	"errors"
	"fmt"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/relay"
	"golang.org/x/net/context"
)

type Organization struct {
	ID           string `json:"id"`
	DatabaseID   string `json:"databaseId"`
	Name         string `json:"name"`
	ResourcePath string `json:"resourcePath"`
}

type User struct {
	ID           string `json:"id"`
	DatabaseID   string `json:"databaseId"`
	Name         string `json:"name"`
	Email        string `json:"email"`
	ResourcePath string `json:"resourcePath"`
}

type Topic struct {
	ID                     string  `json:"id"`
	DatabaseID             string  `json:"databaseId"`
	OrganizationDatabaseID string  `db:"organization_id"`
	Name                   string  `json:"name"`
	Description            *string `json:"description"`
	ResourcePath           string  `json:"resourcePath"`
}

type Resource interface {
	Init()
}

var nodeDefinitions *relay.NodeDefinitions
var OrganizationType *graphql.Object
var UserType *graphql.Object
var QueryType *graphql.Object
var TopicType *graphql.Object

func (o *User) Init() {
	id := relay.ToGlobalID("User", o.DatabaseID)
	o.ResourcePath = fmt.Sprintf("/users/%s", id)
}

func (o *Organization) Init() {
	id := relay.ToGlobalID("Organization", o.DatabaseID)
	o.ResourcePath = fmt.Sprintf("/organizations/%s", id)
}

func (o *Topic) Init() {
	id := relay.ToGlobalID("Topic", o.DatabaseID)
	o.ResourcePath = fmt.Sprintf("/topics/%s", id)
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

func newSchema(conn Connection) (*graphql.Schema, error) {
	nodeDefinitions = relay.NewNodeDefinitions(relay.NodeDefinitionsConfig{
		IDFetcher:   fetcher(conn),
		TypeResolve: resolveType,
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
			"resourcePath": &graphql.Field{
				Type:        graphql.String,
				Description: "The resource path for the user.",
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
			"resourcePath": &graphql.Field{
				Type:        graphql.String,
				Description: "The resource path for the topic.",
			},
		},
		Interfaces: []*graphql.Interface{
			nodeDefinitions.NodeInterface,
		},
	})

	topicConnectionDefinition := relay.ConnectionDefinitions(relay.ConnectionConfig{
		Name:     "Topic",
		NodeType: TopicType,
	})

	OrganizationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Organization",
		Fields: graphql.Fields{
			"id": relay.GlobalIDField("Organization", nil),
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "The name of the organization.",
			},
			"resourcePath": &graphql.Field{
				Type:        graphql.String,
				Description: "The resource path for the organization.",
			},
			"topics": &graphql.Field{
				Type: topicConnectionDefinition.ConnectionType,
				Args: relay.ConnectionArgs,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					args := relay.NewConnectionArguments(p.Args)
					dest := []interface{}{}
					if organization, ok := p.Source.(*Organization); ok {
						err := conn.SelectOrganizationTopics(&dest, organization)
						if err != nil {
							return nil, err
						}
					}
					return relay.ConnectionFromArray(dest, args), nil
				},
			},
		},
		Interfaces: []*graphql.Interface{
			nodeDefinitions.NodeInterface,
		},
	})

	QueryType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"viewer": &graphql.Field{
				Type: UserType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return conn.Viewer()
				},
			},

			"organization": &graphql.Field{
				Type: OrganizationType,

				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Description: "Organization ID",
						Type:        graphql.NewNonNull(graphql.ID),
					},
				},

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return conn.GetOrganization(p.Args["id"].(string))
				},
			},

			"user": &graphql.Field{
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
			},

			"topic": &graphql.Field{
				Type: TopicType,

				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Description: "Topic ID",
						Type:        graphql.NewNonNull(graphql.ID),
					},
				},

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return conn.GetTopic(p.Args["id"].(string))
				},
			},

			"node": nodeDefinitions.NodeField,
		},
	})

	schema, err := graphql.NewSchema(graphql.SchemaConfig{
		Query: QueryType,
	})

	return &schema, err
}
