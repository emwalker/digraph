package resolvers

import (
	"context"
	"errors"
	"log"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type linkResolver struct {
	*Resolver
}

func linkConnection(view *models.View, rows []*models.Link, totalCount int, err error) (models.LinkConnection, error) {
	if err != nil {
		log.Printf("There was a problem constructing link connection: %s", err)
		return models.LinkConnection{}, err
	}

	var edges []*models.LinkEdge
	for _, link := range rows {
		edges = append(edges, &models.LinkEdge{
			Node: models.LinkValue{link, false, view},
		})
	}

	return models.LinkConnection{Edges: edges, TotalCount: totalCount}, nil
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
	ctx context.Context, link *models.LinkValue, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	return availableTopics(ctx, r.DB, link.View, searchString, first, []string{})
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
		return topicConnection(link.View, link.R.ParentTopics, nil)
	}

	log.Print("Fetching parent topics for link")
	mods := []qm.QueryMod{}

	topics, err := link.ParentTopics(mods...).All(ctx, r.DB)
	return topicConnection(link.View, topics, err)
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

// ReviewedAt is the time at which the link was reviewed.
func (r *linkResolver) ViewerReview(ctx context.Context, link *models.LinkValue) (*models.LinkReview, error) {
	var review *models.UserLinkReview
	var err error

	if link.R == nil {
		log.Printf("Fetching reviewedAt for link %s", link.Summary())
		review, err = link.UserLinkReviews(qm.Where("user_id = ?", r.Actor.ID)).One(ctx, r.DB)
		if err != nil {
			if err.Error() == "sql: no rows in result set" {
				return nil, nil
			}
			return nil, err
		}
	} else if len(link.R.UserLinkReviews) == 0 {
		return nil, nil
	} else {
		review = link.R.UserLinkReviews[0]
	}

	reviewedAt := review.ReviewedAt
	if reviewedAt.IsZero() {
		return &models.LinkReview{User: *r.Actor}, nil
	}

	value, err := reviewedAt.Value()
	if err != nil {
		log.Printf("Problem parsing timestamp: %s", err)
		return nil, err
	}

	ts, ok := value.(time.Time)
	if !ok {
		log.Printf("Not a timestamp: %v", ts)
		return nil, errors.New("Expected a timestamp")
	}

	str := ts.Format(time.RFC3339)
	return &models.LinkReview{User: *r.Actor, ReviewedAt: &str}, nil
}
