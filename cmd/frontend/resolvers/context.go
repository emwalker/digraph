package resolvers

import (
	"context"
	"log"
	"sync"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/boil"
)

type key string

// RequestContext holds information about the current request.
type RequestContext struct {
	mux          sync.Mutex
	view         *models.View
	viewer       *models.User
	serverSecret string
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

// ClearRequestSession removes the current request session from the context.
func ClearRequestSession(ctx context.Context) context.Context {
	return context.WithValue(ctx, requestKey, nil)
}

// GetRequestContext returns the current request context.
func GetRequestContext(ctx context.Context) *RequestContext {
	if rc, ok := ctx.Value(requestKey).(*RequestContext); ok {
		return rc
	}
	return GuestRequestContext
}

// InitiatedByServer returns true if the GraphQL operation provides proof that it originated from the
// node server rather than being proxied from the client.
func (c *RequestContext) InitiatedByServer(serverSecret string) bool {
	return c.serverSecret == serverSecret
}

// WithViewer looks up the viewer for the session ID provided, makes sure they match, and adds
// the result to the request context.
func WithViewer(
	ctx context.Context, exec boil.ContextExecutor, viewerID, sessionID string,
) (*models.User, error) {
	var viewer *models.User

	if sessionID == "" {
		viewer = GuestViewer
	} else {
		session, err := models.FindSession(ctx, exec, sessionID)
		if err != nil {
			if err.Error() == queries.ErrSQLNoRows {
				log.Printf("Attempt to query under user %s with bad session id %s", viewerID, sessionID)
				return GuestViewer, nil
			}
			return GuestViewer, errors.Wrap(err, "resolvers: unable to find session")
		}

		viewer, err = session.User().One(ctx, exec)
		if err != nil {
			return GuestViewer, errors.Wrap(err, "resolvers: unable to find user")
		}

		if viewerID != viewer.ID {
			return GuestViewer, errors.Wrap(err, "resolvers: provided viewer id did not match the id of the session user")
		}
	}

	// Add the assumed viewer to the request context
	log.Printf("View with %s and session id %s", viewer, sessionID)
	GetRequestContext(ctx).SetViewer(viewer)

	return viewer, nil
}

// ServerSecret returns the value to be used for proof that a request originates from the server rather
// than being proxied through the client.
func (c *RequestContext) ServerSecret() string {
	return c.serverSecret
}

// SetView sets the current view.
func (c *RequestContext) SetView(view *models.View) {
	c.mux.Lock()
	defer c.mux.Unlock()
	c.view = view
}

// SetViewer sets the current viewer.
func (c *RequestContext) SetViewer(viewer *models.User) {
	c.mux.Lock()
	defer c.mux.Unlock()
	c.viewer = viewer
}

// SetServerSecret adds a secret to the request context that can be used to verify that the request
// originates from the server rather than one that is proxied from the client.
func (c *RequestContext) SetServerSecret(secret string) {
	c.mux.Lock()
	defer c.mux.Unlock()
	c.serverSecret = secret
}

// View returns the current view.
func (c *RequestContext) View() *models.View {
	c.mux.Lock()
	defer c.mux.Unlock()

	if c.view == nil {
		return GuestView
	}
	return c.view
}

// Viewer returns the current viewer.
func (c *RequestContext) Viewer() *models.User {
	c.mux.Lock()
	defer c.mux.Unlock()

	if c.viewer == nil {
		return GuestViewer
	}
	return c.viewer
}

// WithRequestContext adds the specified request context object to the context.
func WithRequestContext(ctx context.Context, rc *RequestContext) context.Context {
	return context.WithValue(ctx, requestKey, rc)
}
