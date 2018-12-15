package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

type repositoryResolver struct {
	*Resolver
}

func (r *repositoryResolver) DisplayName(
	ctx context.Context, repo *models.Repository,
) (string, error) {
	if repo.IsPrivate() {
		return "Private collection", nil
	}
	return repo.Name, nil
}

// isPrivate indicates whether the repository is private or not.
func (r *repositoryResolver) IsPrivate(
	ctx context.Context, repo *models.Repository,
) (bool, error) {
	return repo.IsPrivate(), nil
}

// Organization returns a set of links.
func (r *repositoryResolver) Organization(
	ctx context.Context, repo *models.Repository,
) (models.Organization, error) {
	org, err := repo.Organization().One(ctx, r.DB)
	if err != nil {
		return models.Organization{}, err
	}
	return *org, err
}

// Organization returns a set of links.
func (r *repositoryResolver) Owner(
	ctx context.Context, repo *models.Repository,
) (models.User, error) {
	org, err := repo.Owner().One(ctx, r.DB)
	return *org, err
}

// RootTopic returns the root topic of the repository.
func (r *repositoryResolver) RootTopic(
	ctx context.Context, repo *models.Repository,
) (models.Topic, error) {
	topic, err := repo.RootTopic(ctx, r.DB)
	if err != nil {
		return models.Topic{}, err
	}
	return *topic, err
}
