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
		edges = append(edges, &models.LinkEdge{
			Node: models.LinkValue{link, false},
		})
	}

	return models.LinkConnection{Edges: edges}, nil
}

func linkRepository(ctx context.Context, link *models.LinkValue) (*models.Repository, error) {
	if link.R != nil && link.R.Repository != nil {
		return link.R.Repository, nil
	}
	repo, err := fetchRepository(ctx, link.RepositoryID)
	return &repo, err
}

func linkOrganization(ctx context.Context, link *models.LinkValue) (*models.Organization, error) {
	if link.R != nil && link.R.Organization != nil {
		return link.R.Organization, nil
	}
	repo, err := fetchOrganization(ctx, link.OrganizationID)
	return &repo, err
}

// AvailableParentTopics returns the topics that can be added to the link.
func (r *linkResolver) AvailableParentTopics(
	ctx context.Context, link *models.LinkValue, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	return availableTopics(ctx, r.DB, r.Actor, first)
}

// CreatedAt returns the time at which the link was first added.
func (r *linkResolver) CreatedAt(_ context.Context, link *models.LinkValue) (string, error) {
	return link.CreatedAt.Format(time.RFC3339), nil
}

// Loading is true if the link is being loaded.  Only used on the client.
func (r *linkResolver) Loading(_ context.Context, link *models.LinkValue) (bool, error) {
	return false, nil
}

// Organization returns the organization for a link.
func (r *linkResolver) Organization(ctx context.Context, link *models.LinkValue) (models.Organization, error) {
	org, err := linkOrganization(ctx, link)
	return *org, err
}

// ParentTopics returns the topics under which the link is categorized.
func (r *linkResolver) ParentTopics(
	ctx context.Context, link *models.LinkValue, first *int, after *string, last *int, before *string,
) (models.TopicConnection, error) {
	if link.R != nil && link.R.ParentTopics != nil {
		return topicConnection(link.R.ParentTopics, nil)
	}

	log.Print("Fetching parent topics for link")
	mods := []qm.QueryMod{}

	topics, err := link.ParentTopics(mods...).All(ctx, r.DB)
	return topicConnection(topics, err)
}

// Repository returns the repository of the link.
func (r *linkResolver) Repository(ctx context.Context, link *models.LinkValue) (models.Repository, error) {
	repo, err := linkRepository(ctx, link)
	return *repo, err
}

// ResourcePath returns a path to the item.
func (r *linkResolver) ResourcePath(_ context.Context, link *models.LinkValue) (string, error) {
	return "/links/" + link.ID, nil
}

// Sha1 returns the SHA1 of the normalized url.
func (r *linkResolver) Sha1(_ context.Context, link *models.LinkValue) (string, error) {
	return link.Sha1, nil
}

// Title returns the title of the link.
func (r *linkResolver) Title(_ context.Context, link *models.LinkValue) (string, error) {
	return link.Title, nil
}

// UpdatedAt returns the time of the most recent update.
func (r *linkResolver) UpdatedAt(_ context.Context, link *models.LinkValue) (string, error) {
	return link.UpdatedAt.Format(time.RFC3339), nil
}

// URL returns the title of the link.
func (r *linkResolver) URL(_ context.Context, link *models.LinkValue) (string, error) {
	return link.URL, nil
}
