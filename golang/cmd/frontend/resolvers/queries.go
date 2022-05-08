package resolvers

import (
	"context"
	"errors"
	"log"

	"github.com/emwalker/digraph/golang/cmd/frontend/models"
	perrors "github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

type queryResolver struct{ *Resolver }

// Alerts is a no-op implementation provided as part of a workaround for Relay Modern weirdness
// relating to the swallowing of GraphQL errors.  If the client finds errors in the normal GraphQL
// error field, it will copy them to this field so that components know about them.
//
// - https://github.com/facebook/relay/issues/1913
// - https://github.com/facebook/relay/issues/1913#issuecomment-358636018
func (r *queryResolver) Alerts(ctx context.Context) ([]*models.Alert, error) {
	return []*models.Alert{}, nil
}

// FakeError returns an error on demand in order to facilitate the debugging of error handling in
// the client.
func (r *queryResolver) FakeError(ctx context.Context) (*string, error) {
	return nil, errors.New("there was a problem")
}

func (r *queryResolver) fetchCurrentRepo(
	ctx context.Context, orgLogin string, repoName *string,
) (*models.Repository, error) {
	if orgLogin == "" {
		return nil, errors.New("No current organization login provided")
	}

	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.Where("o.login = ?", orgLogin),
	}

	if repoName == nil {
		log.Printf("Fetching repository for %s/<system>", orgLogin)
		mods = append(mods, qm.Where("repositories.system"))
	} else {
		log.Printf("Fetching repository for %s/%s", orgLogin, *repoName)
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
	ctx context.Context, viewerID, orgLogin string, repoName *string, repositoryIds []string,
	searchString *string,
) (*models.View, error) {
	viewer := GetRequestContext(ctx).Viewer()

	repo, err := r.fetchCurrentRepo(ctx, orgLogin, repoName)
	if err != nil {
		return GuestView, perrors.Wrap(err, "resolvers: unable to fetch current repo")
	}

	view := &models.View{
		ViewerID:                 viewer.ID,
		CurrentOrganizationLogin: orgLogin,
		CurrentRepositoryName:    repoName,
		CurrentRepository:        repo,
		RepositoryIds:            repositoryIds,
		SearchString:             searchString,
	}

	return view, nil
}
