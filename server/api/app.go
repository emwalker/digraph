package api

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/graphql-go/graphql"
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
	_ "github.com/mattes/migrate"
	"github.com/nu7hatch/gouuid"
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
	err := app.Engine.Start(":8080")
	if err != nil {
		log.Fatal(err)
	}
}

func New(conn Connection) (*App, error) {
	schema, err := newSchema(conn)
	if err != nil {
		return nil, err
	}

	err = conn.Init()
	if err != nil {
		return nil, err
	}

	engine := echo.New()

	app := App{
		Connection: conn,
		Engine:     engine,
		Schema:     schema,
	}

	engine.Use(middleware.Recover())

	engine.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:8080", "http://localhost:5001"},
		AllowHeaders: []string{echo.HeaderOrigin, echo.HeaderContentType},
	}))

	engine.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: `${method} | ${status} | ${uri} -> ${latency_human}` + "\n",
		Output: os.Stdout,
	}))

	engine.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			c.Set("app", app)
			id, _ := uuid.NewV4()
			c.Set("uuid", id)
			return next(c)
		}
	})

	engine.POST("/graphql", func(c echo.Context) error {
		return app.HandleGraphqlQuery(c)
	})

	return &app, nil
}
