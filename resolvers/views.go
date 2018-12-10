package resolvers

import (
	"context"
	"log"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type viewResolver struct{ *Resolver }

func topicQueryMods(view *models.View, filter qm.QueryMod, searchString *string) []qm.QueryMod {
	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on topics.organization_id = o.id"),
		qm.WhereIn("o.id in ?", view.OrganizationIdsForQuery()...),
		qm.Load("ParentTopics"),
		qm.Load("ChildTopics"),
		qm.Load("ChildLinks"),
	}

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
	ctx context.Context, view *models.View, linkId string,
) (*models.Link, error) {
	if len(view.OrganizationIds) < 1 {
		return nil, nil
	}

	scope := models.Links(
		qm.InnerJoin("organizations o on links.organization_id = o.id"),
		qm.WhereIn("o.id in ?", view.OrganizationIdsForQuery()...),
		qm.Where("links.id = ?", linkId),
	)
	return scope.One(ctx, r.DB)
}

// Links returns a set of links.
func (r *viewResolver) Links(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string, last *int,
	before *string,
) (*models.LinkConnection, error) {
	if len(view.OrganizationIds) < 1 {
		return linkConnection(models.Links(qm.Where("1 = 0")).All(ctx, r.DB))
	}

	pageSize := 100
	if first != nil {
		pageSize = *first
	}

	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on links.organization_id = o.id"),
		qm.WhereIn("o.id in ?", view.OrganizationIdsForQuery()...),
		qm.OrderBy("created_at desc"),
		qm.Load("ParentTopics"),
		qm.Limit(pageSize),
	}

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ilike '%%' || ? || '%%'", searchString))
	}

	scope := models.Links(mods...)
	return linkConnection(scope.All(ctx, r.DB))
}

// Topic returns a topic for a given id.
func (r *viewResolver) Topic(
	ctx context.Context, view *models.View, topicID string,
) (*models.Topic, error) {
	if len(view.OrganizationIds) < 1 {
		return nil, nil
	}

	log.Printf("Fetching topic %s", topicID)
	scope := models.Topics(topicQueryMods(view, qm.Where("topics.id = ?", topicID), nil)...)
	return scope.One(ctx, r.DB)
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (*models.TopicConnection, error) {
	if len(view.OrganizationIds) < 1 {
		return topicConnection(models.Topics(qm.Where("1 = 0")).All(ctx, r.DB))
	}

	scope := models.Topics(topicQueryMods(view, nil, searchString)...)
	return topicConnection(scope.All(ctx, r.DB))
}
