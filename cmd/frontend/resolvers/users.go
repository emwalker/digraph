package resolvers

import (
	"context"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
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

func (r *userResolver) DefaultRepository(ctx context.Context, user *models.User) (*models.Repository, error) {
	if user.IsGuest() {
		return nil, nil
	}
	return user.DefaultRepo(ctx, r.DB)
}

func (r *userResolver) IsGuest(ctx context.Context, user *models.User) (bool, error) {
	return user.IsGuest(), nil
}

// Email returns the email of a user.
func (r *userResolver) PrimaryEmail(_ context.Context, user *models.User) (string, error) {
	return user.PrimaryEmail, nil
}

// Repositories returns the repositories to which the user has access
func (r *userResolver) Repositories(
	ctx context.Context, user *models.User, first *int, after *string, last *int, before *string,
) (models.RepositoryConnection, error) {
	if user.IsGuest() {
		return models.RepositoryConnection{}, nil
	}

	var edges []*models.RepositoryEdge
	var err error
	var repos []*models.Repository

	selectedID := ""
	if id := user.SelectedRepositoryID.Ptr(); id != nil {
		selectedID = *id
	}

	repos, err = models.Repositories(
		qm.InnerJoin("organizations o on repositories.organization_id = o.id"),
		qm.InnerJoin("organization_members om on o.id = om.organization_id"),
		qm.Where("om.user_id = ?", user.ID),
	).All(ctx, r.DB)
	if err != nil {
		return models.RepositoryConnection{}, err
	}

	for _, repo := range repos {
		edges = append(edges, &models.RepositoryEdge{
			Node:       *repo,
			IsSelected: repo.ID == selectedID,
		})
	}

	return models.RepositoryConnection{Edges: edges}, nil
}

func (r *userResolver) SelectedRepository(
	ctx context.Context, user *models.User,
) (*models.Repository, error) {
	repoID := user.SelectedRepositoryID.Ptr()
	if repoID == nil {
		return nil, nil
	}
	repo, err := fetchRepository(ctx, *repoID)
	return &repo, err
}

// UpdatedAt returns the time of the most recent update.
func (r *userResolver) UpdatedAt(_ context.Context, user *models.User) (string, error) {
	return user.UpdatedAt.Format(time.RFC3339), nil
}
