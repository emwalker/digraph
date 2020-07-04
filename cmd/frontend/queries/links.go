package queries

import (
	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// LinkQuery holds common information for querying for links.
type LinkQuery struct {
	first        *int
	reviewed     *bool
	searchString *string
	view         *models.View
	viewer       *models.User
}

// NewLinkQuery returns an initialized LinkQuery.
func NewLinkQuery(
	view *models.View, viewer *models.User, searchString *string, first *int, reviewed *bool,
) *LinkQuery {
	return &LinkQuery{
		first:        first,
		reviewed:     reviewed,
		searchString: searchString,
		view:         view,
		viewer:       viewer,
	}
}

func (q LinkQuery) pageSize() int {
	if q.first == nil {
		return 100
	}
	return *q.first
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

	mods = q.view.Filter(mods)

	if !q.viewer.IsGuest() {
		mods = append(
			mods,
			qm.Load(
				models.LinkRels.UserLinkReviews, qm.Where("user_link_reviews.user_id = ?", q.viewer.ID), qm.Limit(1),
			),
		)
	}

	if q.reviewed != nil {
		mods = append(
			mods,
			qm.Load(models.LinkRels.UserLinkReviews, qm.Where("user_link_reviews.user_id = ?", q.viewer.ID), qm.Limit(1)),
			qm.InnerJoin("user_link_reviews ulr on links.id = ulr.link_id"),
			qm.Where("ulr.user_id = ?", q.viewer.ID),
		)

		if *q.reviewed {
			mods = append(mods, qm.Where("ulr.reviewed_at is not null"))
		} else {
			mods = append(mods, qm.Where("ulr.reviewed_at is null"))
		}
	}

	if q.searchString != nil && *q.searchString != "" {
		q := NewSearchQuery(*q.searchString)
		array := q.WildcardStringArray()

		mods = append(mods,
			qm.Where("links.title ~~* all(?) or links.url ~~* all(?)", array, array),
		)
	}

	return mods
}
