package api

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/graphql-go/graphql"
	"github.com/labstack/echo"
	_ "github.com/mattes/migrate"
)

func init() {
	log.SetOutput(os.Stdout)
}

type TitleFetcher func(string) (string, error)

type Config struct {
	Address    string
	Connection Connection
	DriverName string
	Engine     *echo.Echo
	FetchTitle TitleFetcher
	Schema     *graphql.Schema
}

type GraphqlRequest struct {
	Query     string                 `json:"query"`
	Variables map[string]interface{} `json:"variables"`
}

func (config *Config) HandleGraphqlQuery(c echo.Context) (err error) {
	log.Println("attempting to read request")
	req := new(GraphqlRequest)
	if err = c.Bind(req); err != nil {
		log.Println("read failed: ", err)
		return
	}

	log.Printf(`querying GraphQL: "%s"`, req.Query)
	result := graphql.Do(graphql.Params{
		Schema:         *config.Schema,
		RequestString:  req.Query,
		VariableValues: req.Variables,
	})

	log.Println("query finished, sending response")
	r := c.Response()
	r.Header().Set(echo.HeaderContentType, echo.MIMEApplicationJSONCharsetUTF8)
	r.WriteHeader(http.StatusOK)
	return json.NewEncoder(c.Response()).Encode(result)
}

func (config *Config) Run() {
	checkErr(config.Engine.Start(":5000"))
}

func New(config *Config) (*Config, error) {
	config.Connection = config.newConnection()
	schema, err := config.newSchema()
	checkErr(err)

	checkErr(config.Connection.Init())
	config.Schema = schema

	if config.Engine != nil {
		config.Engine.POST("/graphql", func(c echo.Context) error {
			return config.HandleGraphqlQuery(c)
		})
	}

	return config, nil
}
