package api

import (
	"errors"
	"fmt"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/relay"
	"golang.org/x/net/context"
)

type Organization struct {
	ID         string `json:"id"`
	DatabaseId string `json:"databaseId"`
	Name       string `json:"name"`
}

type User struct {
	ID         string `json:"id"`
	DatabaseId string `json:"databaseId"`
	Name       string `json:"name"`
	Email      string `json:"email"`
}

var nodeDefinitions *relay.NodeDefinitions
var OrganizationType *graphql.Object
var UserType *graphql.Object
var QueryType *graphql.Object

var Schema graphql.Schema

func findById(id string, info graphql.ResolveInfo, ctx context.Context) (interface{}, error) {
	resolvedID := relay.FromGlobalID(id)

	switch resolvedID.Type {
	case "Organization":
		return connection.GetOrganization(resolvedID.ID)
	case "User":
		return connection.GetUser(resolvedID.ID)
	default:
		return nil, errors.New(fmt.Sprintf("unknown node type: %s", resolvedID.Type))
	}
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

func init() {
	nodeDefinitions = relay.NewNodeDefinitions(relay.NodeDefinitionsConfig{
		IDFetcher:   findById,
		TypeResolve: resolveType,
	})

	UserType = graphql.NewObject(graphql.ObjectConfig{
		Name: "User",
		Fields: graphql.Fields{
			"id": relay.GlobalIDField("User", nil),
			"databaseId": &graphql.Field{
				Type:        graphql.String,
				Description: "The id of the organization.",
			},
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

	OrganizationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Organization",
		Fields: graphql.Fields{
			"id": relay.GlobalIDField("Organization", nil),
			"databaseId": &graphql.Field{
				Type:        graphql.String,
				Description: "The id of the organization.",
			},
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "The name of the organization.",
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
					return connection.Viewer()
				},
			},

			"organization": &graphql.Field{
				Type: OrganizationType,

				Args: graphql.FieldConfigArgument{
					"databaseId": &graphql.ArgumentConfig{
						Description: "Organization ID",
						Type:        graphql.NewNonNull(graphql.ID),
					},
				},

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return connection.GetOrganization(p.Args["databaseId"].(string))
				},
			},

			"user": &graphql.Field{
				Type: UserType,

				Args: graphql.FieldConfigArgument{
					"databaseId": &graphql.ArgumentConfig{
						Description: "User ID",
						Type:        graphql.NewNonNull(graphql.ID),
					},
				},

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return connection.GetUser(p.Args["databaseId"].(string))
				},
			},

			"node": nodeDefinitions.NodeField,
		},
	})

	// mutationType := graphql.NewObject(graphql.ObjectConfig{
	// 	Name: "Mutation",
	// 	Fields: graphql.Fields{
	// 	},
	// })

	var err error
	Schema, err = graphql.NewSchema(graphql.SchemaConfig{
		Query: QueryType,
	})

	if err != nil {
		panic(err)
	}
}
