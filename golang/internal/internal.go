package internal

import (
	"context"

	"github.com/emwalker/digraph/golang/internal/models"
)

type Repo interface {
	GetDefaultReposByOrgID(context.Context, []string) ([]*models.Repository, error)
	GetOrgsByID(context.Context, []string) ([]*models.Organization, error)
	GetReposByID(context.Context, []string) ([]*models.Repository, error)
	GetTopicsByID(context.Context, []string) ([]*models.Topic, error)
}
