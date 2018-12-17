package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/models"
)

type userResolver struct {
	*Resolver
}

// AvatarURL provides a link to a picture of the user.
func (r *userResolver) AvatarURL(_ context.Context, user *models.User) (string, error) {
	url := user.GithubAvatarURL.Ptr()
	if url != nil {
		return *url, nil
	}
	return "", nil
}

// CreatedAt returns of the creation of the user account.
func (r *userResolver) CreatedAt(_ context.Context, user *models.User) (string, error) {
	return user.CreatedAt.Format(time.RFC3339), nil
}

func (r *userResolver) DefaultRepository(
	ctx context.Context, user *models.User,
) (*models.Repository, error) {
	return user.DefaultRepo(ctx, r.DB)
}

// Email returns the email of a user.
func (r *userResolver) PrimaryEmail(_ context.Context, user *models.User) (string, error) {
	return user.PrimaryEmail, nil
}

// UpdatedAt returns the time of the most recent update.
func (r *userResolver) UpdatedAt(_ context.Context, user *models.User) (string, error) {
	return user.UpdatedAt.Format(time.RFC3339), nil
}
