package resolvers

import (
	"context"
	"log"
	"time"

	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type linkResolver struct {
	*Resolver
}

func linkConnection(rows []*models.Link, err error) (models.LinkConnection, error) {
	if err != nil {
		return models.LinkConnection{}, err
	}

	var edges []*models.LinkEdge
	for _, link := range rows {
		edges = append(edges, &models.LinkEdge{Node: *link})
	}

	return models.LinkConnection{Edges: edges}, nil
}

// AvailableParentTopics returns the topics that can be added to the link.
func (r *linkResolver) AvailableParentTopics(
	ctx context.Context, link *models.Link, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	existingTopics, err := link.ParentTopics(qm.Select("id")).All(ctx, r.DB)
	if err != nil {
		return models.TopicConnection{}, err
	}

	var existingIds []interface{}
	for _, topic := range existingTopics {
		existingIds = append(existingIds, topic.ID)
	}

	org, err := fetchOrganization(ctx, link.OrganizationID)
	if err != nil {
		return models.TopicConnection{}, err
	}

	topics, err := org.Topics().All(ctx, r.DB)
	return topicConnection(nil, topics, err)
}

// CreatedAt returns the time at which the link was first added.
func (r *linkResolver) CreatedAt(_ context.Context, link *models.Link) (string, error) {
	return link.CreatedAt.Format(time.RFC3339), nil
}

// Organization returns the organization for a link.
func (r *linkResolver) Organization(
	ctx context.Context, link *models.Link,
) (models.Organization, error) {
	return fetchOrganization(ctx, link.OrganizationID)
}

// ResourcePath returns a path to the item.
func (r *linkResolver) ResourcePath(_ context.Context, link *models.Link) (string, error) {
	return "/links/" + link.ID, nil
}

// ParentTopics returns the topics under which the link is categorized.
func (r *linkResolver) ParentTopics(
	ctx context.Context, link *models.Link, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	if link.R != nil && link.R.ParentTopics != nil {
		return topicConnection(nil, link.R.ParentTopics, nil)
	}

	log.Print("Fetching parent topics for link")
	mods := []qm.QueryMod{
		qm.Load("Repository"),
	}

	topics, err := link.ParentTopics(mods...).All(ctx, r.DB)
	return topicConnection(nil, topics, err)
}

// UpdatedAt returns the time of the most recent update.
func (r *linkResolver) UpdatedAt(_ context.Context, link *models.Link) (string, error) {
	return link.UpdatedAt.Format(time.RFC3339), nil
}
