package queries

import (
	"context"
	"log"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/queries/parser"
	"github.com/emwalker/digraph/golang/internal/util"
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
	mods := q.View.Filter([]qm.QueryMod{
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
		qm.Limit(q.pageSize()),
	})
	parentTopicMods := []qm.QueryMod{}

	if q.Viewer.IsGuest() {
		// Filter out parent topics that are not publicly visible
		parentTopicMods = append(
			parentTopicMods,
			qm.LeftOuterJoin("repositories r on topics.repository_id = r.id"),
			qm.LeftOuterJoin("organizations o on r.organization_id = o.id"),
			qm.Where("o.public"),
		)
	} else {
		mods = append(
			mods,
			qm.Load(
				models.LinkRels.UserLinkReviews,
				qm.Where("user_link_reviews.user_id = ?", q.Viewer.ID),
				qm.Limit(1),
			),
		)

		// Filter out parent topics that the viewer does not have permission to see
		parentTopicMods = append(
			parentTopicMods,
			qm.InnerJoin("repositories r on topics.repository_id = r.id"),
			qm.InnerJoin("organizations o on r.organization_id = o.id"),
			qm.InnerJoin("organization_members om on om.organization_id = o.id"),
			qm.Where("om.user_id = ?", q.Viewer.ID),
		)
	}

	mods = append(mods, qm.Load("ParentTopics", parentTopicMods...))

	if q.Reviewed != nil {
		mods = append(
			mods,
			qm.Load(
				models.LinkRels.UserLinkReviews,
				qm.Where("user_link_reviews.user_id = ?", q.Viewer.ID),
				qm.Limit(1),
			),
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

// LinkParentTopics returns the parent topics of a topic.
type LinkParentTopics struct {
	*models.View
	Link *models.LinkValue
}

// Fetch fetches the parent topics
func (q LinkParentTopics) Fetch(ctx context.Context, exec boil.ContextExecutor) ([]*models.Topic, error) {
	log.Printf("Fetching parent topics for topic %s", q.Link)
	mods := q.Filter([]qm.QueryMod{
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.OrderBy("topics.name"),
	})

	return q.Link.ParentTopics(mods...).All(ctx, exec)
}
