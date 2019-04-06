package services

import (
	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/volatiletech/sqlboiler/boil"
)

// PublicOrgID holds the id of the public organization.
const PublicOrgID = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb"

// Connection holds fields used by service calls.
type Connection struct {
	Exec    boil.ContextExecutor
	Actor   *models.User
	Fetcher pageinfo.Fetcher
}

// CleanupFunc is a function that can be called to roll back the effects of a service call.
type CleanupFunc func() error

// New returns a new service connection
func New(
	exec boil.ContextExecutor, actor *models.User, fetcher pageinfo.Fetcher,
) *Connection {
	return &Connection{exec, actor, fetcher}
}
