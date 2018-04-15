package api

import (
	"github.com/graphql-go/graphql"
)

var QueryType = graphql.NewObject(graphql.ObjectConfig{
	Name: "Query",
	Fields: graphql.Fields{
		"viewer": &graphql.Field{
			Type: UserType,

			Resolve: func(p graphql.ResolveParams) (interface{}, error) {
				return conn.GetViewer()
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
				id := p.Args["id"].(string)
				return conn.GetOrganizationByID(id)
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
				id := p.Args["id"].(string)
				return conn.GetUserByID(id)
			},
		},
	},
})
