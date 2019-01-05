package resolvers

import (
	"context"
	"log"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type viewResolver struct{ *Resolver }

func newViewFromTopic(ctx context.Context, exec boil.ContextExecutor, topic *models.Topic) (*models.View, error) {
	repo, err := fetchRepository(ctx, topic.RepositoryID)
	if err != nil {
		return nil, err
	}

	org, err := fetchOrganization(ctx, repo.OrganizationID)
	if err != nil {
		return nil, err
	}

	return &models.View{
		CurrentOrganizationLogin: org.Login,
		CurrentRepositoryName:    &repo.Name,
		CurrentRepository:        &repo,
	}, nil
}

func pageSizeOrDefault(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}

func addRepos(mods []qm.QueryMod, view *models.View) []qm.QueryMod {
	if view.RepositoriesSelected() {
		mods = append(mods, qm.WhereIn("r.id in ?", view.RepositoryIdsForQuery()...))
	} else if view.ViewerID == "" {
		mods = append(mods,
			qm.InnerJoin("organizations o on o.id = r.organization_id"),
			qm.Where("r.system and o.public"),
		)
	} else {
		mods = append(mods, qm.Where("r.system and r.owner_id = ?", view.ViewerID))
	}
	return mods
}

func topicQueryMods(view *models.View, filter qm.QueryMod, searchString *string, first *int) []qm.QueryMod {
	mods := []qm.QueryMod{
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
	}
	mods = addRepos(mods, view)

	if filter != nil {
		mods = append(mods, filter)
	}

	if searchString != nil {
		mods = append(mods, qm.Where("topics.name ilike '%%' || ? || '%%'", *searchString))
	}

	return mods
}

// Link returns a specific link.
func (r *viewResolver) Link(
	ctx context.Context, view *models.View, linkID string,
) (*models.LinkValue, error) {
	mods := []qm.QueryMod{
		qm.Where("links.id = ?", linkID),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	}
	mods = addRepos(mods, view)
	link, err := models.Links(mods...).One(ctx, r.DB)
	return &models.LinkValue{link, false}, err
}

// Links returns a set of links.
func (r *viewResolver) Links(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.LinkConnection, error) {
	mods := []qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	}
	mods = addRepos(mods, view)

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ilike '%%' || ? || '%%'", searchString))
	}

	scope := models.Links(mods...)
	return linkConnection(scope.All(ctx, r.DB))
}

func (r *viewResolver) CurrentRepository(
	ctx context.Context, view *models.View,
) (*models.Repository, error) {
	return view.CurrentRepository, nil
}

// Topic returns a topic for a given id.
func (r *viewResolver) Topic(
	ctx context.Context, view *models.View, topicID string,
) (*models.TopicValue, error) {
	log.Printf("Fetching topic %s", topicID)
	scope := models.Topics(topicQueryMods(view, qm.Where("topics.id = ?", topicID), nil, nil)...)
	topic, err := scope.One(ctx, r.DB)
	return &models.TopicValue{topic, false}, err
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	topics, err := models.Topics(topicQueryMods(view, nil, searchString, first)...).All(ctx, r.DB)
	return topicConnection(topics, err)
}
