package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

type userResolver struct {
	*Resolver
}

// Email returns the email of a user.
func (r *userResolver) PrimaryEmail(_ context.Context, user *models.User) (string, error) {
	return user.PrimaryEmail, nil
}

// SelectedTopic returns the user's currently selected topic.
func (r *userResolver) SelectedTopic(_ context.Context, user *models.User) (*models.Topic, error) {
	return nil, nil
}
