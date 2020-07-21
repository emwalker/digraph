package queries

import (
	"context"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries/parser"
	"github.com/emwalker/digraph/cmd/frontend/util"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// LinkQuery holds common information for querying for links.
type LinkQuery struct {
	First              *int
	IncludeDescendants bool
	Reviewed           *bool
	SearchString       *string
	Topic              *models.Topic
	View               *models.View
	Viewer             *models.User
}

func (q LinkQuery) pageSize() int {
	if q.First == nil {
		return 100
	}
	return *q.First
}

// Mods returns a set of query mods that can be used for querying for links.
func (q LinkQuery) Mods() []qm.QueryMod {
	mods := []qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.Limit(q.pageSize()),
	}

	mods = append(
		mods,
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	)

	mods = q.View.Filter(mods)

	if !q.Viewer.IsGuest() {
		mods = append(
			mods,
			qm.Load(
				models.LinkRels.UserLinkReviews, qm.Where("user_link_reviews.user_id = ?", q.Viewer.ID), qm.Limit(1),
			),
		)
	}

	if q.Reviewed != nil {
		mods = append(
			mods,
			qm.Load(models.LinkRels.UserLinkReviews, qm.Where("user_link_reviews.user_id = ?", q.Viewer.ID), qm.Limit(1)),
			qm.InnerJoin("user_link_reviews ulr on links.id = ulr.link_id"),
			qm.Where("ulr.user_id = ?", q.Viewer.ID),
		)

		if *q.Reviewed {
			mods = append(mods, qm.Where("ulr.reviewed_at is not null"))
		} else {
			mods = append(mods, qm.Where("ulr.reviewed_at is null"))
		}
	}

	if util.Present(q.SearchString) {
		search := parser.Parse(q.SearchString)
		array := search.WildcardStringArray()
		mods = append(mods,
			qm.Where("links.title ~~* all(?) or links.url ~~* all(?)", array, array),
		)
	}

	if q.Topic != nil {
		if q.IncludeDescendants {
			mods = append(
				mods,
				qm.InnerJoin("link_transitive_closure lte on links.id = lte.child_id"),
				qm.Where("lte.parent_id = ?", q.Topic.ID),
			)
		} else {
			mods = append(
				mods,
				qm.InnerJoin("link_topics lt on lt.child_id = links.id"),
				qm.Where("lt.parent_id = ?", q.Topic.ID),
			)
		}
	}

	return mods
}

// Fetch fetches the query
func (q LinkQuery) Fetch(ctx context.Context, exec boil.ContextExecutor) ([]*models.Link, error) {
	mods := append(
		q.Mods(),
		qm.GroupBy("links.id"),
		qm.OrderBy("links.created_at desc"),
	)
	return models.Links(mods...).All(ctx, exec)
}
