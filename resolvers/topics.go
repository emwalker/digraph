package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

type topicResolver struct {
	*Resolver
}

// Description returns a description of the topic.
func (r *topicResolver) Description(_ context.Context, topic *models.Topic) (*string, error) {
	return topic.Description.Ptr(), nil
}

// Organization returns a set of links.
func (r *topicResolver) Organization(ctx context.Context, topic *models.Topic) (models.Organization, error) {
	org, err := topic.Organization().One(ctx, r.DB)
	return *org, err
}

// ResourcePath returns a path to the item.
func (r *topicResolver) ResourcePath(_ context.Context, topic *models.Topic) (string, error) {
	return "/topics/" + topic.ID, nil
}

func topicConnection(rows []*models.Topic, err error) (*models.TopicConnection, error) {
	if err != nil {
		return nil, err
	}

	var edges []*models.TopicEdge
	for _, topic := range rows {
		edges = append(edges, &models.TopicEdge{Node: *topic})
	}

	return &models.TopicConnection{Edges: edges}, nil
}

// ChildTopics returns a set of topics.
func (r *topicResolver) ChildTopics(ctx context.Context, topic *models.Topic, first *int, after *string, last *int, before *string) (*models.TopicConnection, error) {
	return topicConnection(topic.ChildTopics().All(ctx, r.DB))
}

// ParentTopics returns a set of links.
func (r *topicResolver) ParentTopics(ctx context.Context, topic *models.Topic, first *int, after *string, last *int, before *string) (*models.TopicConnection, error) {
	return topicConnection(topic.ParentTopics().All(ctx, r.DB))
}

// Links returns a set of links.
func (r *topicResolver) Links(ctx context.Context, topic *models.Topic, first *int, after *string, last *int, before *string) (*models.LinkConnection, error) {
	return linkConnection(topic.ChildLinks().All(ctx, r.DB))
}
