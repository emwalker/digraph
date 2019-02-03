package resolvers

import (
	"context"
	"log"

	"github.com/emwalker/digraph/internal/models"
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

func topicQueryMods(view *models.View, filter qm.QueryMod, searchString *string, first *int) []qm.QueryMod {
	mods := view.Filter([]qm.QueryMod{
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
	})

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
	mods := view.Filter([]qm.QueryMod{
		qm.Where("links.id = ?", linkID),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	link, err := models.Links(mods...).One(ctx, r.DB)
	return &models.LinkValue{link, false, view}, err
}

// Links returns a set of links.
func (r *viewResolver) Links(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.LinkConnection, error) {
	mods := view.Filter([]qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ilike '%%' || ? || '%%'", searchString))
	}

	scope := models.Links(mods...)
	conn, err := scope.All(ctx, r.DB)
	return linkConnection(view, conn, err)
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
	return &models.TopicValue{topic, false, view}, err
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	topics, err := models.Topics(topicQueryMods(view, nil, searchString, first)...).All(ctx, r.DB)
	return topicConnection(view, topics, err)
}
