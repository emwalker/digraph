package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

// CreateTopic creates a new topic.
func (r *mutationResolver) CreateTopic(ctx context.Context, input models.CreateTopicInput) (*models.CreateTopicPayload, error) {
	panic("not implemented")
}

// SelectTopic updates the currently selected topic.
func (r *mutationResolver) SelectTopic(ctx context.Context, input models.SelectTopicInput) (*models.SelectTopicPayload, error) {
	panic("not implemented")
}

// UpsertLink adds a new link to the database.
func (r *mutationResolver) UpsertLink(ctx context.Context, input models.UpsertLinkInput) (*models.UpsertLinkPayload, error) {
	panic("not implemented")
}
