package resolvers

import (
	"context"
	"errors"
	"log"

	"github.com/emwalker/digraph/models"
	// "github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type queryResolver struct{ *Resolver }

// Viewer returns the logged-in user.
func (r *queryResolver) Viewer(ctx context.Context) (*models.User, error) {
	return getCurrentUser(ctx), nil
}

func (r *queryResolver) fetchCurrentRepo(
	ctx context.Context, orgLogin string, repoName *string,
) (*models.Repository, error) {
	if orgLogin == "" {
		return nil, errors.New("No current organization login provided")
	}

	log.Printf("Fetching repository for %s/%v", orgLogin, repoName)
	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.Where("o.login = ?", orgLogin),
	}

	if repoName == nil {
		mods = append(mods, qm.Where("repositories.system"))
	} else {
		mods = append(mods, qm.Where("repositories.name = ?", repoName))
	}

	repo, err := models.Repositories(mods...).One(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	log.Print("Repository found")
	return repo, nil
}

// View returns a resolver that filters results on the basis of one or more organizations.
func (r *queryResolver) View(
	ctx context.Context, orgLogin string, repoName *string, repositoryIds []string, viewerID *string,
) (models.View, error) {
	if viewerID == nil {
		var viewer *models.User
		if viewer = getCurrentUser(ctx); viewer == nil {
			return models.View{}, errors.New("No viewer has been provided")
		}
		viewerID = &viewer.ID
	}

	repo, err := r.fetchCurrentRepo(ctx, orgLogin, repoName)
	if err != nil {
		return models.View{}, err
	}

	view := models.View{
		CurrentOrganizationLogin: orgLogin,
		CurrentRepositoryName:    repoName,
		CurrentRepository:        repo,
		ViewerID:                 *viewerID,
		RepositoryIds:            repositoryIds,
	}

	return view, nil
}
