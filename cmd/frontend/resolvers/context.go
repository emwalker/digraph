package resolvers

import (
	"context"
	"sync"

	"github.com/emwalker/digraph/cmd/frontend/models"
)

type key string

// RequestContext holds information about the current request.
type RequestContext struct {
	viewMu   sync.Mutex
	view     *models.View
	viewerMu sync.Mutex
	viewer   *models.User
}

var requestKey key = "requestKey"

// Default constants for use in a guest context.
var (
	GuestViewer         = &models.User{Name: "Anonymous", ID: ""}
	GuestView           = &models.View{}
	GuestRequestContext = &RequestContext{view: GuestView, viewer: GuestViewer}
)

// NewRequestContext returns a new *RequestContext object initialized with the parameters
// passed in.
func NewRequestContext(viewer *models.User) *RequestContext {
	return &RequestContext{viewer: viewer}
}

// WithRequestContext adds the specified request context object to the context.
func WithRequestContext(ctx context.Context, rc *RequestContext) context.Context {
	return context.WithValue(ctx, requestKey, rc)
}

// GetRequestContext returns the current request context.
func GetRequestContext(ctx context.Context) *RequestContext {
	if rc, ok := ctx.Value(requestKey).(*RequestContext); ok {
		return rc
	}
	return GuestRequestContext
}

// View returns the current view.
func (c *RequestContext) View() *models.View {
	c.viewerMu.Lock()
	defer c.viewerMu.Unlock()

	if c.view == nil {
		return GuestView
	}
	return c.view
}

// Viewer returns the current viewer.
func (c *RequestContext) Viewer() *models.User {
	c.viewerMu.Lock()
	defer c.viewerMu.Unlock()

	if c.viewer == nil {
		return GuestViewer
	}
	return c.viewer
}

// SetView sets the current view.
func (c *RequestContext) SetView(view *models.View) {
	c.viewMu.Lock()
	defer c.viewMu.Unlock()
	c.view = view
}

// SetViewer sets the current viewer.
func (c *RequestContext) SetViewer(viewer *models.User) {
	c.viewerMu.Lock()
	defer c.viewerMu.Unlock()
	c.viewer = viewer
}
