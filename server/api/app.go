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

type App struct {
	Connection Connection
	Engine     *echo.Echo
	Schema     *graphql.Schema
}

type GraphqlRequest struct {
	Query     string                 `json:"query"`
	Variables map[string]interface{} `json:"variables"`
}

func (app *App) HandleGraphqlQuery(c echo.Context) (err error) {
	log.Println("attempting to read request")
	req := new(GraphqlRequest)
	if err = c.Bind(req); err != nil {
		log.Println("read failed: ", err)
		return
	}

	log.Printf(`querying GraphQL: "%s"`, req.Query)
	result := graphql.Do(graphql.Params{
		Schema:         *app.Schema,
		RequestString:  req.Query,
		VariableValues: req.Variables,
	})

	log.Println("query finished, sending response")
	r := c.Response()
	r.Header().Set(echo.HeaderContentType, echo.MIMEApplicationJSONCharsetUTF8)
	r.WriteHeader(http.StatusOK)
	return json.NewEncoder(c.Response()).Encode(result)
}

func (app *App) Run() {
	err := app.Engine.Start(":5000")
	if err != nil {
		log.Fatal(err)
	}
}

func New(conn Connection, engine *echo.Echo) (*App, error) {
	schema, err := newSchema(conn)
	checkErr(err)

	checkErr(conn.Init())

	app := App{
		Connection: conn,
		Engine:     engine,
		Schema:     schema,
	}

	engine.POST("/graphql", func(c echo.Context) error {
		return app.HandleGraphqlQuery(c)
	})

	return &app, nil
}
