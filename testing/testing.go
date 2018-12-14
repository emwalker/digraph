package testing

import (
	"context"

	"github.com/emwalker/digraph/services"
)

type CleanupFunc func() error

func CreateUser(
	c services.Connection, ctx context.Context, name, email, githubUsername, githubAvatarURL string,
) (*services.CreateUserResult, CleanupFunc, error) {
	result, err := c.CreateUser(ctx, name, email, githubUsername, githubAvatarURL)

	if err != nil {
		return nil, nil, err
	}

	cleanup := func() error {
		result.User.Delete(ctx, c.Exec)
		result.Organization.Delete(ctx, c.Exec)
		return nil
	}

	return result, cleanup, nil
}
