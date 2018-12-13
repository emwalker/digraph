package resolvers

import (
	"context"
	"log"
	"math"
	"time"

	"github.com/emwalker/digraph/loaders"
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func getTopicLoader(ctx context.Context) *loaders.TopicLoader {
	return ctx.Value(loaders.TopicLoaderKey).(*loaders.TopicLoader)
}

type topicResolver struct {
	*Resolver
}

func topicConnection(rows []*models.Topic, err error) (models.TopicConnection, error) {
	if err != nil {
		return models.TopicConnection{}, err
	}

	var edges []*models.TopicEdge
	for _, topic := range rows {
		edges = append(edges, &models.TopicEdge{Node: *topic})
	}

	return models.TopicConnection{Edges: edges}, nil
}

// AvailableParentTopics returns the topics that can be added to the link.
func (r *topicResolver) AvailableParentTopics(
	ctx context.Context, topic *models.Topic, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	existingTopics, err := topic.ParentTopics(qm.Select("id")).All(ctx, r.DB)
	if err != nil {
		return models.TopicConnection{}, err
	}

	var existingIds []interface{}
	for _, topic := range existingTopics {
		existingIds = append(existingIds, topic.ID)
	}

	org, err := topic.Organization().One(ctx, r.DB)
	if err != nil {
		return models.TopicConnection{}, err
	}

	return topicConnection(org.Topics().All(ctx, r.DB))
}

// ChildTopics returns a set of topics.
func (r *topicResolver) ChildTopics(
	ctx context.Context, topic *models.Topic, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	mods := []qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.OrderBy("name"),
	}

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("name ilike '%%' || ? || '%%'", *searchString))
	}

	return topicConnection(topic.ChildTopics(mods...).All(ctx, r.DB))
}

// CreatedAt returns the time of the topic's creation.
func (r *topicResolver) CreatedAt(_ context.Context, topic *models.Topic) (string, error) {
	return topic.CreatedAt.Format(time.RFC3339), nil
}

// Description returns a description of the topic.
func (r *topicResolver) Description(_ context.Context, topic *models.Topic) (*string, error) {
	return topic.Description.Ptr(), nil
}

// Links returns a set of links.
func (r *topicResolver) Links(
	ctx context.Context, topic *models.Topic, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.LinkConnection, error) {
	mods := []qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.OrderBy("created_at desc"),
	}

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ilike '%%' || ? || '%%'", searchString))
	}

	scope := topic.ChildLinks(mods...)
	return linkConnection(scope.All(ctx, r.DB))
}

// Organization returns a set of links.
func (r *topicResolver) Organization(
	ctx context.Context, topic *models.Topic,
) (models.Organization, error) {
	org, err := topic.Organization().One(ctx, r.DB)
	return *org, err
}

// ParentTopics returns a set of links.
func (r *topicResolver) ParentTopics(
	ctx context.Context, topic *models.Topic, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	if topic.R != nil && topic.R.ParentTopics != nil {
		return topicConnection(topic.R.ParentTopics, nil)
	}

	log.Printf("Fetching parent topics for topic %s", topic.ID)
	return topicConnection(topic.ParentTopics(qm.OrderBy("name")).All(ctx, r.DB))
}

// Repository returns the repostory of the topic.
func (r *topicResolver) Repository(
	ctx context.Context, topic *models.Topic,
) (models.Repository, error) {
	org, err := topic.Repository().One(ctx, r.DB)
	return *org, err
}

// ResourcePath returns a path to the item.
func (r *topicResolver) ResourcePath(_ context.Context, topic *models.Topic) (string, error) {
	return "/topics/" + topic.ID, nil
}

func (r *topicResolver) matchingDescendantTopics(
	ctx context.Context, topic *models.Topic, searchString string, limit int,
) ([]*models.Topic, error) {
	var rows []struct {
		ID string
	}

	err := queries.Raw(`
	with recursive child_topics as (
	  select parent_id, child_id
	  from topic_topics where parent_id = $1
	union
	  select pt.child_id, ct.child_id
	  from topic_topics ct
	  inner join child_topics pt on pt.child_id = ct.parent_id
	)
	select t.id from topics t
	inner join child_topics ct on ct.child_id = t.id
	where t.name ilike '%%' || $2 || '%%'
	limit $3
	`, topic.ID, searchString, limit).Bind(ctx, r.DB, &rows)

	if err != nil {
		return nil, err
	}

	if len(rows) < 1 {
		return nil, nil
	}

	var ids []interface{}
	for _, row := range rows {
		ids = append(ids, row.ID)
	}

	return models.Topics(qm.Load("ParentTopics"), qm.WhereIn("id in ?", ids...)).All(ctx, r.DB)
}

func (r *topicResolver) matchingDescendantLinks(
	ctx context.Context, topic *models.Topic, searchString string, limit int,
) ([]*models.Link, error) {
	var rows []struct {
		ID string
	}

	err := queries.Raw(`
	with recursive child_topics as (
	  select parent_id, parent_id as child_id
	  from topic_topics where parent_id = $1
	union
	  select pt.child_id, ct.child_id
	  from topic_topics ct
	  inner join child_topics pt on pt.child_id = ct.parent_id
	)
	select l.id from links l
	inner join link_topics lt on l.id = lt.child_id
	inner join child_topics ct on ct.child_id = lt.parent_id
	where l.title ilike '%%' || $2 || '%%'
	limit $3
	`, topic.ID, searchString, limit).Bind(ctx, r.DB, &rows)

	if err != nil {
		return nil, err
	}

	if len(rows) < 1 {
		return nil, nil
	}

	var ids []interface{}
	for _, row := range rows {
		ids = append(ids, row.ID)
	}

	return models.Links(qm.Load("ParentTopics"), qm.WhereIn("id in ?", ids...)).All(ctx, r.DB)
}

func (r *topicResolver) Search(
	ctx context.Context, topic *models.Topic, searchString string, first *int, after *string,
	last *int, before *string,
) (models.SearchResultItemConnection, error) {
	log.Printf("Searching topic %s for '%s'", topic.ID, searchString)

	var (
		err    error
		topics []*models.Topic
		links  []*models.Link
		edges  []*models.SearchResultItemEdge
	)

	var limit = math.MaxInt32
	if first != nil {
		limit = *first
	}

	if topics, err = r.matchingDescendantTopics(ctx, topic, searchString, limit); err != nil {
		return models.SearchResultItemConnection{}, err
	}

	limit -= len(topics)
	if links, err = r.matchingDescendantLinks(ctx, topic, searchString, limit); err != nil {
		return models.SearchResultItemConnection{}, err
	}

	for _, topic := range topics {
		edges = append(edges, &models.SearchResultItemEdge{Node: *topic})
	}

	for _, link := range links {
		edges = append(edges, &models.SearchResultItemEdge{Node: *link})
	}

	return models.SearchResultItemConnection{Edges: edges}, nil
}

// UpdatedAt returns the time of the most recent update.
func (r *topicResolver) UpdatedAt(_ context.Context, topic *models.Topic) (string, error) {
	return topic.UpdatedAt.Format(time.RFC3339), nil
}
