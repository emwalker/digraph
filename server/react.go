package main

import (
	"html/template"
	"net/http"
	"time"

	"github.com/labstack/echo"
	"github.com/nu7hatch/gouuid"
)

// React struct is contains JS vms
// pool to serve HTTP requests and
// separates some domain specific
// resources.
type React struct {
	debug bool
	path  string
}

// NewReact initialized React struct
func NewReact(filePath string, debug bool, proxy http.Handler) *React {
	r := &React{
		debug: debug,
		path:  filePath,
	}
	return r
}

// Handle handles all HTTP requests which
// have not been caught via static file
// handler or other middlewares.
func (r *React) Handle(c echo.Context) error {
	UUID := c.Get("uuid").(*uuid.UUID)
	defer func() {
		if r := recover(); r != nil {
			c.Render(http.StatusInternalServerError, "react.html", Resp{
				UUID:  UUID.String(),
				Error: r.(string),
			})
		}
	}()

	return c.Render(http.StatusOK, "react.html", Resp{
		UUID: UUID.String(),
	})
}

// Resp is a struct for convinient
// react app Response parsing.
// Feel free to add any other keys to this struct
// and return value for this key at ecmascript side.
// Keep it sync with: src/app/client/router/toString.js:23
type Resp struct {
	UUID       string        `json:"uuid"`
	Error      string        `json:"error"`
	Redirect   string        `json:"redirect"`
	App        string        `json:"app"`
	Title      string        `json:"title"`
	Meta       string        `json:"meta"`
	Initial    string        `json:"initial"`
	RenderTime time.Duration `json:"-"`
}

// HTMLApp returns a application template
func (r Resp) HTMLApp() template.HTML {
	return template.HTML(r.App)
}

// HTMLTitle returns a title data
func (r Resp) HTMLTitle() template.HTML {
	return template.HTML(r.Title)
}

// HTMLMeta returns a meta data
func (r Resp) HTMLMeta() template.HTML {
	return template.HTML(r.Meta)
}
