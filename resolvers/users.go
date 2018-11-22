package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/models"
)

type userResolver struct {
	*Resolver
}

// CreatedAt returns of the creation of the user account.
func (r *userResolver) CreatedAt(_ context.Context, user *models.User) (string, error) {
	return user.CreatedAt.Format(time.RFC3339), nil
}

// Email returns the email of a user.
func (r *userResolver) PrimaryEmail(_ context.Context, user *models.User) (string, error) {
	return user.PrimaryEmail, nil
}

// SelectedTopic returns the user's currently selected topic.
func (r *userResolver) SelectedTopic(_ context.Context, user *models.User) (*models.Topic, error) {
	return nil, nil
}

// UpdatedAt returns the time of the most recent update.
func (r *userResolver) UpdatedAt(_ context.Context, user *models.User) (string, error) {
	return user.UpdatedAt.Format(time.RFC3339), nil
}
