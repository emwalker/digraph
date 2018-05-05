package api

import (
	"errors"
	"fmt"
	"log"
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
	Description *string  `json:"description" quad:"di:description,optional"`
}

type Link struct {
	_          struct{} `quad:"@type > di:link"`
	ID         string   `json:"id" quad:",optional"`
	ResourceID quad.IRI `json:"@id"`
	Title      string   `json:"title" quad:"di:title"`
	URL        string   `json:"url" quad:"di:url"`
	TopicIDs   []string `quad:",optional"`
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

func NewLink(node *Link, conn Connection) *Link {
	var useTitle string

	if node.Title == "" {
		var err error
		useTitle, err = conn.FetchTitle(node.URL)
		if err != nil {
			useTitle = node.URL
		}
		node.Title = useTitle
	}

	node.ResourceID = generateIDForType("link")
	node.Init()
	return node
}

func NewTopic(node *Topic) *Topic {
	node.ResourceID = generateIDForType("topic")
	node.Init()
	return node
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

func (config *Config) fetcher() relay.IDFetcherFn {
	return func(id string, info graphql.ResolveInfo, ctx context.Context) (interface{}, error) {
		resolvedID := relay.FromGlobalID(id)
		orgId := quad.IRI("organization:tyrell")

		switch resolvedID.Type {
		case "Organization":
			return config.Connection.FetchOrganization(resolvedID.ID)
		case "User":
			return config.Connection.FetchUser(resolvedID.ID)
		case "Link":
			return config.Connection.FetchLink(orgId, resolvedID.ID)
		case "Topic":
			return config.Connection.FetchTopic(orgId, resolvedID.ID)
		default:
			return nil, errors.New(fmt.Sprintf("unknown node type: %s", resolvedID.Type))
		}
	}
}

func (config *Config) createTopicMutation(edgeType graphql.Output) *graphql.Field {
	return relay.MutationWithClientMutationID(relay.MutationConfig{
		Name: "CreateTopic",

		InputFields: graphql.InputObjectConfigFieldMap{
			"organizationId": &graphql.InputObjectFieldConfig{
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
					payload := p.Source.(map[string]interface{})
					orgId := payload["organizationId"].(quad.IRI)
					node, err := config.Connection.FetchTopic(orgId, payload["topicId"].(string))
					checkErr(err)
					return &relay.Edge{Node: node}, nil
				},
			},
		},

		MutateAndGetPayload: func(input map[string]interface{}, info graphql.ResolveInfo, ctx context.Context) (map[string]interface{}, error) {
			orgId := quad.IRI(input["organizationId"].(string))
			node := NewTopic(&Topic{
				Name:        input["name"].(string),
				Description: maybeString(input["description"]),
			})
			checkErr(config.Connection.CreateTopic(orgId, node))

			return map[string]interface{}{
				"topicId":        node.ID,
				"organizationId": orgId,
			}, nil
		},
	})
}

func (config *Config) createLinkMutation(edgeType graphql.Output) *graphql.Field {
	return relay.MutationWithClientMutationID(relay.MutationConfig{
		Name: "CreateLink",

		InputFields: graphql.InputObjectConfigFieldMap{
			"organizationId": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"title": &graphql.InputObjectFieldConfig{
				Type:         graphql.String,
				DefaultValue: nil,
			},
			"url": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"topicIds": &graphql.InputObjectFieldConfig{
				Type:         graphql.NewList(graphql.String),
				DefaultValue: []interface{}{},
			},
		},

		OutputFields: graphql.Fields{
			"linkEdge": &graphql.Field{
				Type: edgeType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					payload := p.Source.(map[string]interface{})
					orgId := payload["organizationId"].(quad.IRI)
					node, err := config.Connection.FetchLink(orgId, payload["linkId"].(string))
					checkErr(err)
					return &relay.Edge{Node: node}, nil
				},
			},
		},

		MutateAndGetPayload: func(input map[string]interface{}, info graphql.ResolveInfo, ctx context.Context) (map[string]interface{}, error) {
			var topicIds []string
			if ids, ok := input["topicIds"].([]interface{}); ok {
				for _, topicId := range ids {
					topicIds = append(topicIds, topicId.(string))
				}
			}

			orgId := quad.IRI(input["organizationId"].(string))
			node := NewLink(&Link{
				URL:      input["url"].(string),
				Title:    stringOr("", input["title"]),
				TopicIDs: topicIds,
			}, config.Connection)
			checkErr(config.Connection.CreateLink(orgId, node))

			return map[string]interface{}{
				"linkId":         node.ID,
				"organizationId": orgId,
			}, nil
		},
	})
}

func (config *Config) selectTopicMutation(topicType *Type) *graphql.Field {
	return relay.MutationWithClientMutationID(relay.MutationConfig{
		Name: "SelectTopic",

		InputFields: graphql.InputObjectConfigFieldMap{
			"organizationId": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
			"topicId": &graphql.InputObjectFieldConfig{
				Type: graphql.String,
			},
		},

		OutputFields: graphql.Fields{
			"topic": &graphql.Field{
				Type: topicType.NodeType,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					payload := p.Source.(map[string]interface{})
					if topicId, ok := payload["topicId"].(string); ok {
						orgId := payload["organizationId"].(quad.IRI)
						return config.Connection.FetchTopic(orgId, topicId)
					}
					return nil, nil
				},
			},
		},

		MutateAndGetPayload: func(
			input map[string]interface{},
			info graphql.ResolveInfo,
			ctx context.Context,
		) (map[string]interface{}, error) {
			viewer, err := config.Connection.Viewer()
			checkErr(err)
			orgId := quad.IRI(input["organizationId"].(string))
			topicId := input["topicId"].(string)

			node, err := config.Connection.SelectTopic(orgId, viewer.(*User).ID, topicId)
			if err != nil {
				log.Println("there was a problem:", err)
				return map[string]interface{}{}, nil
			}

			return map[string]interface{}{
				"topicId":        node.ID,
				"organizationId": orgId,
			}, nil
		},
	})
}

func (config *Config) viewerField(userType *Type) *graphql.Field {
	return &graphql.Field{
		Type: userType.NodeType,

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return config.Connection.Viewer()
		},
	}
}

func (config *Config) newSchema() (*graphql.Schema, error) {
	nodeDefinitions = relay.NewNodeDefinitions(relay.NodeDefinitionsConfig{
		IDFetcher:   config.fetcher(),
		TypeResolve: resolveType,
	})

	topicType = NewType(&TypeConfig{
		Name: "Topic",

		FetchNode: func(orgId quad.IRI, resourceId string) (interface{}, error) {
			return config.Connection.FetchTopic(orgId, resourceId)
		},

		FetchConnection: func(orgId quad.IRI, out *[]interface{}) {
			checkErr(config.Connection.FetchTopics(orgId, out))
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

			"selectedTopic": &graphql.Field{
				Type:        topicType.NodeType,
				Description: "Topic selected by the user",

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					user := p.Source.(*User)
					orgId := quad.IRI("organization:tyrell")
					return config.Connection.SelectedTopic(orgId, user.ID)
				},
			},
		},

		NodeDefinitions: nodeDefinitions,
	})

	linkType = NewType(&TypeConfig{
		Name: "Link",

		FetchNode: func(orgId quad.IRI, resourceId string) (interface{}, error) {
			return config.Connection.FetchLink(orgId, resourceId)
		},

		FetchConnection: func(orgId quad.IRI, out *[]interface{}) {
			checkErr(config.Connection.FetchLinks(orgId, out))
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

			"topics": &graphql.Field{
				Type: topicType.Definitions.ConnectionType,

				Args: relay.ConnectionArgs,

				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					args := relay.NewConnectionArguments(p.Args)
					dest := []interface{}{}
					link := p.Source.(*Link)
					orgId := quad.IRI("organization:tyrell")
					config.Connection.FetchTopicsForLink(orgId, &dest, link)
					return relay.ConnectionFromArray(dest, args), nil
				},
			},
		},

		NodeDefinitions: nodeDefinitions,
	})

	topicType.NodeType.AddFieldConfig("links", &graphql.Field{
		Type: linkType.Definitions.ConnectionType,

		Args: relay.ConnectionArgs,

		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			args := relay.NewConnectionArguments(p.Args)
			dest := []interface{}{}
			topic := p.Source.(*Topic)
			orgId := quad.IRI("organization:tyrell")
			config.Connection.FetchLinksForTopic(orgId, &dest, topic)
			return relay.ConnectionFromArray(dest, args), nil
		},
	})

	organizationType = NewType(&TypeConfig{
		Name: "Organization",

		FetchNode: func(orgId quad.IRI, resourceId string) (interface{}, error) {
			return config.Connection.FetchOrganization(resourceId)
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
			"viewer":       config.viewerField(userType),
			"organization": organizationType.NodeField,
			"user":         userType.NodeField,
			"node":         nodeDefinitions.NodeField,
		},
	})

	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"createTopic": config.createTopicMutation(topicType.Definitions.EdgeType),
			"createLink":  config.createLinkMutation(linkType.Definitions.EdgeType),
			"selectTopic": config.selectTopicMutation(topicType),
		},
	})

	schema, err := graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})

	return &schema, err
}
