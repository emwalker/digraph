package resolvers

import (
	"context"
	"log"
	"time"
	"unicode/utf8"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/resolvers/activity"
	"github.com/go-redis/redis"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

var (
	topicGraphKey = "topicGraph"
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
		qm.Load("ParentTopics"),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
	})

	if filter != nil {
		mods = append(mods, filter)
	}

	if searchString != nil {
		mods = append(mods, qm.Where("topics.name ~~* all(?)", wildcardStringArray(*searchString)))
	}

	return mods
}

// Activity returns a feed of actions that have recently been taken.
func (r *viewResolver) Activity(
	ctx context.Context, view *models.View, first *int, after *string, last *int, before *string,
) (models.ActivityLineItemConnection, error) {
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
		log.Printf("There was a problem fetching user link records: %s", err)
		return models.ActivityLineItemConnection{}, nil
	}

	logData := make([]activity.UpsertLink, len(userLinks))

	for i, userLink := range userLinks {
		topics := make([]activity.Topic, len(userLink.R.UserLinkTopics))

		for j, linkTopic := range userLink.R.UserLinkTopics {
			topic := linkTopic.R.Topic
			topics[j] = activity.Topic{topic.Name, topic.ID}
		}

		logData[i] = activity.UpsertLink{
			CreatedAt: userLink.CreatedAt,
			User:      activity.User{userLink.R.User.Name},
			Link:      activity.Link{userLink.R.Link.Title, userLink.R.Link.URL},
			Topics:    topics,
		}
	}

	edges, err := activity.MakeEdges(logData)

	return models.ActivityLineItemConnection{Edges: edges}, err
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
) (models.LinkConnection, error) {
	mods := view.Filter([]qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ~~* all(?)", wildcardStringArray(*searchString)))
	}

	if reviewed != nil {
		mods = append(
			mods,
			qm.Load(models.LinkRels.UserLinkReviews, qm.Where("user_link_reviews.user_id = ?", r.Actor.ID), qm.Limit(1)),
			qm.InnerJoin("user_link_reviews ulr on links.id = ulr.link_id"),
			qm.Where("ulr.user_id = ?", r.Actor.ID),
		)

		if *reviewed {
			mods = append(mods, qm.Where("ulr.reviewed_at is not null"))
		} else {
			mods = append(mods, qm.Where("ulr.reviewed_at is null"))
		}
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

// TopicCount returns the number of topics the general collection.  Eventually it wll return the
// number of topics in the view.
func (r *viewResolver) TopicCount(ctx context.Context, view *models.View) (int, error) {
	count, err := models.Topics(qm.Where("topics.repository_id = ?", generalRepositoryID)).Count(ctx, r.DB)
	return int(count), err
}

// TopicGraph returns a json string that can be used for building a visual representation of the graph.
func (r *viewResolver) TopicGraph(ctx context.Context, view *models.View) (*string, error) {
	cachedGraph, err := r.RD.Get(topicGraphKey).Result()
	if err != nil && err != redis.Nil {
		return nil, err
	}

	if err == nil {
		log.Printf("Returning cached topic graph at '%s'", topicGraphKey)
		return &cachedGraph, nil
	}

	log.Printf("Querying for topic graph and setting '%s' redis key", topicGraphKey)

	result := struct {
		Payload string
	}{}

	// TODO - search within the repositories specified in view.RepositoryIds
	err = queries.Raw(`
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
	`, generalRepositoryID).Bind(ctx, r.DB, &result)

	if err != nil {
		log.Printf("There was a problem fetching topic graph: %s", err)
		return nil, err
	}

	log.Printf("Setting '%s' redis key with %d char payload", topicGraphKey, utf8.RuneCountInString(result.Payload))
	_, err = r.RD.SetNX(topicGraphKey, result.Payload, 10*time.Minute).Result()
	if err != nil {
		log.Printf("There was a problem setting the '%s' redis key: %s", topicGraphKey, err)
		return nil, err
	}

	return &result.Payload, nil
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	topics, err := models.Topics(topicQueryMods(view, nil, searchString, first)...).All(ctx, r.DB)
	return topicConnection(view, topics, err)
}
