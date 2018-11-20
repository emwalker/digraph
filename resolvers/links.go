package resolvers

import (
	"context"

	"github.com/emwalker/digraph/models"
)

type linkResolver struct {
	*Resolver
}

// Description returns a description of the link.
func (r *linkResolver) Title(_ context.Context, link *models.Link) (string, error) {
	return link.Title, nil
}

// Description returns a description of the link.
func (r *linkResolver) Sha1(_ context.Context, link *models.Link) (string, error) {
	return link.Sha1, nil
}

// Organization returns a set of links.
func (r *linkResolver) Organization(ctx context.Context, link *models.Link) (models.Organization, error) {
	org, err := link.Organization().One(ctx, r.DB)
	return *org, err
}

// ResourcePath returns a path to the item.
func (r *linkResolver) ResourcePath(_ context.Context, link *models.Link) (string, error) {
	return "/links/" + link.ID, nil
}

func linkConnection(rows []*models.Link, err error) (*models.LinkConnection, error) {
	if err != nil {
		return nil, err
	}

	var edges []*models.LinkEdge
	for _, link := range rows {
		edges = append(edges, &models.LinkEdge{Node: *link})
	}

	return &models.LinkConnection{Edges: edges}, nil
}

func (r *linkResolver) Topics(ctx context.Context, link *models.Link, first *int, after *string, last *int, before *string) (*models.TopicConnection, error) {
	return topicConnection(link.ParentTopics().All(ctx, r.DB))
}
