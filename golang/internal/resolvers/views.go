package resolvers

import (
	"context"
	"log"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/queries"
	"github.com/emwalker/digraph/golang/internal/queries/parser"
	"github.com/emwalker/digraph/golang/internal/redis"
	"github.com/emwalker/digraph/golang/internal/resolvers/activity"
	"github.com/emwalker/digraph/golang/internal/util"
	"github.com/pkg/errors"
	"github.com/vmihailenco/msgpack/v5"
	squeries "github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

type viewResolver struct{ *Resolver }

func pageSizeOrDefault(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}

func topicQueryMods(view *models.View, filter qm.QueryMod, searchString *string, first *int) []qm.QueryMod {
	mods := view.Filter([]qm.QueryMod{
		qm.Limit(pageSizeOrDefault(first)),
		qm.Load("ParentTopics"),
		qm.Load("Repository"),
		qm.Load("Repository.Owner"),
		qm.Load("Timerange"),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
	})

	if filter != nil {
		mods = append(mods, filter)
	}

	if util.Present(searchString) {
		s := parser.Parse(searchString)
		mods = append(mods, qm.Where("topics.name ~~* all(?)", s.WildcardStringArray()))
	}

	return mods
}

// Activity returns a feed of actions that have recently been taken.
func (r *viewResolver) Activity(
	ctx context.Context, view *models.View, first *int, after *string, last *int, before *string,
) (*models.ActivityLineItemConnection, error) {
	mods := view.Filter([]qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Load("UserLinkTopics"),
		qm.Load("UserLinkTopics.Topic"),
		qm.Load("Link"),
		qm.Load("User"),
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on user_links.repository_id = r.id"),
	})

	userLinks, err := models.UserLinks(mods...).All(ctx, r.DB)
	if err != nil {
		return nil, errors.Wrap(err, "resolvers.Activity")
	}

	logData := make([]activity.UpsertLink, len(userLinks))

	for i, userLink := range userLinks {
		topics := make([]activity.Topic, len(userLink.R.UserLinkTopics))

		for j, linkTopic := range userLink.R.UserLinkTopics {
			topic := linkTopic.R.Topic
			topics[j] = activity.Topic{Name: topic.Name, ID: topic.ID}
		}

		logData[i] = activity.UpsertLink{
			CreatedAt: userLink.CreatedAt,
			User:      activity.User{Name: userLink.R.User.DisplayName()},
			Link:      activity.Link{Title: userLink.R.Link.Title, URL: userLink.R.Link.URL},
			Topics:    topics,
		}
	}

	edges, err := activity.MakeEdges(logData)
	if err != nil {
		return nil, errors.Wrap(err, "resolvers.Activity")
	}

	return &models.ActivityLineItemConnection{Edges: edges}, nil
}

// DefaultOrganization returns the main repository that people are directed to.
func (r *viewResolver) DefaultOrganization(
	ctx context.Context, view *models.View,
) (*models.Organization, error) {
	org, err := models.Organizations(qm.Where("public and login = 'wiki'")).One(ctx, r.DB)
	if err != nil {
		return nil, err
	}
	return org, nil
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
	return &models.LinkValue{Link: link, NewlyAdded: false, View: view}, err
}

// LinkCount returns the number of links the general collection.  Eventually it wll return the
// number of links in the view.
func (r *viewResolver) LinkCount(ctx context.Context, view *models.View) (int, error) {
	count, err := models.Links(qm.Where("links.repository_id = ?", generalRepositoryID)).Count(ctx, r.DB)
	return int(count), err
}

// Links returns a set of links.
func (r *viewResolver) Links(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string, reviewed *bool,
) (*models.LinkConnection, error) {
	viewer := GetRequestContext(ctx).Viewer()

	query := queries.LinkQuery{
		First:        first,
		Reviewed:     reviewed,
		SearchString: searchString,
		View:         view,
		Viewer:       viewer,
	}
	mods := query.Mods()

	totalCount, err := models.Links(mods...).Count(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	mods = append(mods, qm.OrderBy("links.created_at desc"))

	rows, err := models.Links(mods...).All(ctx, r.DB)
	return linkConnection(view, rows, int(totalCount), err)
}

func (r *viewResolver) CurrentRepository(
	ctx context.Context, view *models.View,
) (*models.Repository, error) {
	return view.CurrentRepository, nil
}

// QueryInfo returns information about the query used to build the current view
func (r *viewResolver) QueryInfo(ctx context.Context, view *models.View) (*models.QueryInfo, error) {
	var edges []*models.TopicEdge
	query := parser.Parse(view.SearchString)

	explicitIds := query.ExplicitTopicIds()
	if len(explicitIds) > 0 {
		log.Printf("Fetching info for search topics: %v", query)
		mods := view.Filter([]qm.QueryMod{
			qm.Limit(10),
			qm.InnerJoin("repositories r on topics.repository_id = r.id"),
			qm.WhereIn("topics.id in ?", explicitIds...),
		})

		topics, err := models.Topics(mods...).All(ctx, r.DB)
		if err != nil {
			log.Printf("Unable to fetch topics for search %#v: %s", view.SearchString, err)
			return nil, err
		}

		for _, t := range topics {
			topicValue := models.TopicValue{Topic: t, NewlyAdded: false, View: view}
			edges = append(edges, &models.TopicEdge{Node: &topicValue})
		}
	}

	return &models.QueryInfo{
		Topics:       &models.TopicConnection{Edges: edges},
		StringTokens: query.StringTokens,
	}, nil
}

// SearchString returns a search string, if one was used
func (r *viewResolver) SearchString(ctx context.Context, view *models.View) (*string, error) {
	return view.SearchString, nil
}

// Topic returns a topic for a given id.
func (r *viewResolver) Topic(
	ctx context.Context, view *models.View, topicID string,
) (*models.TopicValue, error) {
	log.Printf("resolvers: fetching topic %s", topicID)

	topic, err := models.Topics(
		topicQueryMods(view, qm.Where("topics.id = ?", topicID), nil, nil)...,
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("resolvers: problem fetching topic %s: %s", topicID, err)
		return nil, err
	}

	return &models.TopicValue{Topic: topic, NewlyAdded: false, View: view}, nil
}

// TopicCount returns the number of topics the general collection.  Eventually it wll return the
// number of topics in the view.
func (r *viewResolver) TopicCount(ctx context.Context, view *models.View) (int, error) {
	count, err := models.Topics(qm.Where("topics.repository_id = ?", generalRepositoryID)).Count(ctx, r.DB)
	return int(count), err
}

// TopicGraph returns a json string that can be used for building a visual representation of the graph.
func (r *viewResolver) TopicGraph(ctx context.Context, view *models.View) (*string, error) {
	type topicResult struct {
		Payload string
	}

	key := redis.NewKey("TopicGraph", "static-value")

	result := r.Redis.Fetch(ctx, key, func() (*string, error) {
		topics := topicResult{}

		// TODO - search within the repositories specified in view.RepositoryIds
		err := squeries.Raw(`
		select jsonb_build_object('links', (
		  select jsonb_agg(a) from (
		    select tt.parent_id source, tt.child_id target, count(distinct lt.child_id) "linkCount"
		    from topic_topics tt
		    join topics t on tt.parent_id = t.id
		    join topics t2 on tt.child_id = t2.id
		    left join link_topics lt on tt.child_id = lt.parent_id
		    where t.repository_id = $1 and t2.repository_id = $1
		    group by tt.parent_id, tt.child_id
		  ) a
		)) ||
		jsonb_build_object('nodes', (
		  select jsonb_agg(b) from (
		    select
		      t.id, t.name, count(distinct lt.child_id) "linkCount",
		      count(distinct tt.child_id) "topicCount"
		    from topics t
		    left join topic_topics tt on t.id = tt.parent_id
		    left join link_topics lt on t.id = lt.parent_id
		    where t.repository_id = $1
		    group by t.id, t.name
		    order by t.id, t.name
		  ) b
		)) payload
		`, generalRepositoryID).Bind(ctx, r.DB, &topics)
		if err != nil {
			return nil, err
		}

		bytes, err := msgpack.Marshal(&topics)
		if err != nil {
			return nil, err
		}

		str := string(bytes)
		return &str, err
	})

	if result.Success() {
		var topics topicResult
		if err := msgpack.Unmarshal([]byte(*result.Payload), &topics); err != nil {
			return nil, err
		}
		return &topics.Payload, nil
	}

	return nil, result.Err
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (*models.TopicConnection, error) {
	mods := topicQueryMods(view, nil, searchString, first)
	mods = append(mods, qm.OrderBy("char_length(topics.name), topics.name"))
	topics, err := models.Topics(mods...).All(ctx, r.DB)
	return topicConnection(view, topics, err)
}

// Viewer returns the logged-in user.
func (r *viewResolver) Viewer(ctx context.Context, view *models.View) (*models.User, error) {
	return GetRequestContext(ctx).Viewer(), nil
}
