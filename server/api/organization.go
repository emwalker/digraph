package api

import (
	"github.com/graphql-go/graphql"
)

type Organization struct {
	ID   string
	Name string
}

var OrganizationType = graphql.NewObject(graphql.ObjectConfig{
	Name: "Organization",

	Fields: graphql.Fields{
		"id": &graphql.Field{
			Type: graphql.NewNonNull(graphql.ID),
			Resolve: func(p graphql.ResolveParams) (interface{}, error) {
				if object, ok := p.Source.(*Organization); ok == true {
					return object.ID, nil
				}
				return nil, nil
			},
		},

		"name": &graphql.Field{
			Type: graphql.NewNonNull(graphql.String),
			Resolve: func(p graphql.ResolveParams) (interface{}, error) {
				if object, ok := p.Source.(*Organization); ok == true {
					return object.Name, nil
				}
				return nil, nil
			},
		},
	},
})
