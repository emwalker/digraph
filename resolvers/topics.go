package resolvers

import (
	"context"
	"fmt"
	"log"
	"math"
	"time"

	"github.com/emwalker/digraph/loaders"
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type topicResolver struct {
	*Resolver
}

func getTopicLoader(ctx context.Context) *loaders.TopicLoader {
	return ctx.Value(loaders.TopicLoaderKey).(*loaders.TopicLoader)
}

func topicConnection(view *models.View, rows []*models.Topic, err error) (models.TopicConnection, error) {
	if err != nil {
		return models.TopicConnection{}, err
	}

	var edges []*models.TopicEdge
	for _, topic := range rows {
		edges = append(edges, &models.TopicEdge{Node: models.TopicValue{topic, view}})
	}

	return models.TopicConnection{Edges: edges}, nil
}

func topicRepository(ctx context.Context, topic *models.TopicValue) (*models.Repository, error) {
	if topic.R != nil && topic.R.Repository != nil {
		return topic.R.Repository, nil
	}
	repo, err := fetchRepository(ctx, topic.RepositoryID)
	return &repo, err
}

func topicOrganization(ctx context.Context, topic *models.TopicValue) (*models.Organization, error) {
	if topic.R != nil && topic.R.Organization != nil {
		return topic.R.Organization, nil
	}
	org, err := fetchOrganization(ctx, topic.OrganizationID)
	return &org, err
}

// AvailableParentTopics returns the topics that can be added to the link.
func (r *topicResolver) AvailableParentTopics(
	ctx context.Context, topic *models.TopicValue, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	existingTopics, err := topic.ParentTopics(qm.Select("id")).All(ctx, r.DB)
	if err != nil {
		return models.TopicConnection{}, err
	}

	var existingIds []interface{}
	for _, topic := range existingTopics {
		existingIds = append(existingIds, topic.ID)
	}

	org, err := topicOrganization(ctx, topic)
	if err != nil {
		return models.TopicConnection{}, err
	}

	topics, err := org.Topics().All(ctx, r.DB)
	return topicConnection(topic.View, topics, err)
}

func (r *topicResolver) BelongsToCurrentRepository(
	ctx context.Context, topic *models.TopicValue,
) (bool, error) {
	return topic.View.CurrentRepository.ID == topic.RepositoryID, nil
}

// ChildTopics returns a set of topics.
func (r *topicResolver) ChildTopics(
	ctx context.Context, topic *models.TopicValue, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	mods := []qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.OrderBy("name"),
	}

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("name ilike '%%' || ? || '%%'", *searchString))
	}

	topics, err := topic.ChildTopics(mods...).All(ctx, r.DB)
	return topicConnection(topic.View, topics, err)
}

// CreatedAt returns the time of the topic's creation.
func (r *topicResolver) CreatedAt(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.CreatedAt.Format(time.RFC3339), nil
}

// Description returns a description of the topic.
func (r *topicResolver) Description(_ context.Context, topic *models.TopicValue) (*string, error) {
	return topic.Description.Ptr(), nil
}

// DisplayColor returns a color by which to display the topic.
func (r *topicResolver) DisplayColor(
	ctx context.Context, topic *models.TopicValue,
) (string, error) {
	color := topic.DisplayColor()
	return color, nil
}

func (r *topicResolver) ID(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.ID, nil
}

// Links returns a set of links.
func (r *topicResolver) Links(
	ctx context.Context, topic *models.TopicValue, searchString *string, first *int, after *string,
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

func (r *topicResolver) Name(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.Name, nil
}

// Organization returns an organization.
func (r *topicResolver) Organization(
	ctx context.Context, topic *models.TopicValue,
) (models.Organization, error) {
	org, err := topicOrganization(ctx, topic)
	return *org, err
}

// ParentTopics returns a set of topics.
func (r *topicResolver) ParentTopics(
	ctx context.Context, topic *models.TopicValue, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	if topic.R != nil && topic.R.ParentTopics != nil {
		return topicConnection(topic.View, topic.R.ParentTopics, nil)
	}

	log.Printf("Fetching parent topics for topic %s", topic.ID)
	mods := []qm.QueryMod{
		qm.OrderBy("name"),
	}

	topics, err := topic.ParentTopics(mods...).All(ctx, r.DB)
	return topicConnection(topic.View, topics, err)
}

// Repository returns the repostory of the topic.
func (r *topicResolver) Repository(
	ctx context.Context, topic *models.TopicValue,
) (models.Repository, error) {
	repo, err := topicRepository(ctx, topic)
	return *repo, err
}

// ResourcePath returns a path to the item.
func (r *topicResolver) ResourcePath(ctx context.Context, topic *models.TopicValue) (string, error) {
	repo, err := topicRepository(ctx, topic)
	if err != nil {
		return "", err
	}

	org, err := topicOrganization(ctx, topic)
	if err != nil {
		return "", err
	}

	if repo.System {
		return fmt.Sprintf("/%s/topics/%s", org.Login, topic.ID), nil
	}
	return fmt.Sprintf("/%s/%s/topics/%s", org.Login, repo.Name, topic.ID), nil
}

func (r *topicResolver) matchingDescendantTopics(
	ctx context.Context, topic *models.TopicValue, searchString string, limit int,
) ([]*models.TopicValue, error) {
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

	topics, err := models.Topics(qm.WhereIn("id in ?", ids...)).All(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	var topicValues []*models.TopicValue
	for _, t := range topics {
		topicValues = append(topicValues, &models.TopicValue{t, topic.View})
	}

	return topicValues, nil
}

func (r *topicResolver) matchingDescendantLinks(
	ctx context.Context, topic *models.TopicValue, searchString string, limit int,
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

	return models.Links(
		qm.Load("ParentTopics"),
		qm.WhereIn("id in ?", ids...),
	).All(ctx, r.DB)
}

func (r *topicResolver) Search(
	ctx context.Context, topic *models.TopicValue, searchString string, first *int, after *string,
	last *int, before *string,
) (models.SearchResultItemConnection, error) {
	log.Printf("Searching topic %s for '%s'", topic.ID, searchString)

	var (
		err    error
		topics []*models.TopicValue
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
func (r *topicResolver) UpdatedAt(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.UpdatedAt.Format(time.RFC3339), nil
}
