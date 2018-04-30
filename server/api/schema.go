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
	_          struct{} `quad:"@type > foaf:Organization"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Name       string   `json:"name" quad:"di:name"`
}

type User struct {
	_          struct{} `quad:"@type > foaf:Person"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Name       string   `json:"name" quad:"di:name"`
	Email      string   `json:"email" quad:"di:email"`
}

type Topic struct {
	_           struct{} `quad:"@type > foaf:topic"`
	ID          string   `json:"id" quad:",optional"`
	ResourceID  quad.IRI `json:"@id"`
	Name        string   `json:"name" quad:"di:name"`
	Description *string  `json:"description" quad:"description,optional"`
}

type Link struct {
	_          struct{} `quad:"@type > di:link"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Title      string   `json:"title" quad:"di:title"`
	URL        string   `json:"url" quad:"di:url"`
}

type Resource interface {
	Init()
	IRI() quad.IRI
}

var (
	linkType         *Type
	nodeDefinitions  *relay.NodeDefinitions
	organizationType *Type
	topicType        *Type
	userType         *Type
)

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

func (o *Link) Init() {
	o.ID = isomorphicID(o.ResourceID)
}

func (o *Link) IRI() quad.IRI {
	return o.ResourceID
}

func resolveType(p graphql.ResolveTypeParams) *graphql.Object {
	switch p.Value.(type) {
	case *Link:
		return linkType.NodeType
	case *Organization:
		return organizationType.NodeType
	case *Topic:
		return topicType.NodeType
	case *User:
		return userType.NodeType
	default:
		panic("unknown type")
	}
}

func fetcher(conn Connection) relay.IDFetcherFn {
	return func(id string, info graphql.ResolveInfo, ctx context.Context) (interface{}, error) {
		resolvedID := relay.FromGlobalID(id)

		switch resolvedID.Type {
		case "Link":
			return conn.FetchLink(resolvedID.ID)
		case "Organization":
			return conn.FetchOrganization(resolvedID.ID)
		case "Topic":
			return conn.FetchTopic(resolvedID.ID)
		case "User":
			return conn.FetchUser(resolvedID.ID)
		default:
			return nil, errors.New(fmt.Sprintf("unknown node type: %s", resolvedID.Type))
		}
	}
}

func createTopicMutation(conn Connection, edgeType graphql.Output) *graphql.Field {
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
				Type: edgeType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if payload, ok := p.Source.(map[string]interface{}); ok {
						node, err := conn.FetchTopic(payload["topicResourceId"].(string))
						checkErr(err)
						return &relay.Edge{Node: node}, nil
					}
					return nil, nil
				},
			},
		},

		MutateAndGetPayload: func(input map[string]interface{}, info graphql.ResolveInfo, ctx context.Context) (map[string]interface{}, error) {
			orgIri := input["organizationResourceId"].(string)
			name := input["name"].(string)
			description := maybeString(input["description"])
			node, err := conn.CreateTopic(orgIri, name, description)
			checkErr(err)

			return map[string]interface{}{
				"topicResourceId": node.ID,
			}, nil
		},
	})
}

func createLinkMutation(conn Connection, edgeType graphql.Output) *graphql.Field {
	return relay.MutationWithClientMutationID(relay.MutationConfig{
		Name: "CreateLink",

		InputFields: graphql.InputObjectConfigFieldMap{
			"organizationResourceId": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"title": &graphql.InputObjectFieldConfig{
				Type:         graphql.String,
				DefaultValue: nil,
			},
			"url": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
		},

		OutputFields: graphql.Fields{
			"linkEdge": &graphql.Field{
				Type: edgeType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if payload, ok := p.Source.(map[string]interface{}); ok {
						node, err := conn.FetchLink(payload["linkResourceId"].(string))
						checkErr(err)
						return &relay.Edge{Node: node}, nil
					}
					return nil, nil
				},
			},
		},

		MutateAndGetPayload: func(input map[string]interface{}, info graphql.ResolveInfo, ctx context.Context) (map[string]interface{}, error) {
			orgIri := input["organizationResourceId"].(string)
			url := input["url"].(string)

			var useTitle string
			if title, ok := input["title"].(string); ok {
				useTitle = title
			} else {
				useTitle = url
			}

			node, err := conn.CreateLink(orgIri, url, useTitle)
			checkErr(err)

			return map[string]interface{}{
				"linkResourceId": node.ID,
			}, nil
		},
	})
}

func newSchema(conn Connection) (*graphql.Schema, error) {
	nodeDefinitions = relay.NewNodeDefinitions(relay.NodeDefinitionsConfig{
		IDFetcher:   fetcher(conn),
		TypeResolve: resolveType,
	})

	userType = NewType(&TypeConfig{
		Name: "User",

		NodeFields: graphql.Fields{
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "Name of the user",
			},
			"email": &graphql.Field{
				Type:        graphql.String,
				Description: "Email address of the user",
			},
		},

		NodeDefinitions: nodeDefinitions,
	})

	topicType = NewType(&TypeConfig{
		Name: "Topic",

		FetchNode: func(resourceId string) (interface{}, error) {
			return conn.FetchTopic(resourceId)
		},

		FetchConnection: func(out *[]interface{}, org *Organization) {
			checkErr(conn.FetchTopics(out, org))
		},

		NodeFields: graphql.Fields{
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "Name of the topic",
			},
			"description": &graphql.Field{
				Type:        graphql.String,
				Description: "Description of the topic",
			},
		},

		NodeDefinitions: nodeDefinitions,
	})

	linkType = NewType(&TypeConfig{
		Name: "Link",

		FetchNode: func(resourceId string) (interface{}, error) {
			return conn.FetchLink(resourceId)
		},

		FetchConnection: func(out *[]interface{}, org *Organization) {
			checkErr(conn.FetchLinks(out, org))
		},

		NodeFields: graphql.Fields{
			"title": &graphql.Field{
				Type:        graphql.String,
				Description: "Title of the page",
			},
			"url": &graphql.Field{
				Type:        graphql.String,
				Description: "Url of the page",
			},
		},

		NodeDefinitions: nodeDefinitions,
	})

	organizationType = NewType(&TypeConfig{
		Name: "Organization",

		FetchNode: func(resourceId string) (interface{}, error) {
			return conn.FetchOrganization(resourceId)
		},

		NodeFields: graphql.Fields{
			"name": &graphql.Field{
				Type:        graphql.String,
				Description: "Name of the organization",
			},
			"topic":  topicType.NodeField,
			"topics": topicType.ConnectionField,
			"links":  linkType.ConnectionField,
		},

		NodeDefinitions: nodeDefinitions,
	})

	queryType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"viewer": &graphql.Field{
				Type: userType.NodeType,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return conn.Viewer()
				},
			},
			"organization": organizationType.NodeField,
			"user":         userType.NodeField,
			"node":         nodeDefinitions.NodeField,
		},
	})

	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"createTopic": createTopicMutation(conn, topicType.Definitions.EdgeType),
			"createLink":  createLinkMutation(conn, linkType.Definitions.EdgeType),
		},
	})

	schema, err := graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})

	return &schema, err
}
