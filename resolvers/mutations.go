package resolvers

import (
	"context"
	"crypto/sha1"
	"fmt"

	pl "github.com/PuerkitoBio/purell"
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/boil"
)

// CreateTopic creates a new topic.
func (r *MutationResolver) CreateTopic(ctx context.Context, input models.CreateTopicInput) (*models.CreateTopicPayload, error) {
	panic("not implemented")
}

// SelectTopic updates the currently selected topic.
func (r *MutationResolver) SelectTopic(ctx context.Context, input models.SelectTopicInput) (*models.SelectTopicPayload, error) {
	panic("not implemented")
}

type URL struct {
	CanonicalURL string
	Input        string
	Sha1         string
}

func normalizeUrl(url string) (*URL, error) {
	canonical, err := pl.NormalizeURLString(url, pl.FlagsUsuallySafeNonGreedy)
	if err != nil {
		return nil, err
	}

	sha1 := fmt.Sprintf("%x", sha1.Sum([]byte(canonical)))
	return &URL{canonical, url, sha1}, nil
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(ctx context.Context, input models.UpsertLinkInput) (*models.UpsertLinkPayload, error) {
	url, err := normalizeUrl(input.URL)
	if err != nil {
		return nil, err
	}

	link := &models.Link{
		OrganizationID: input.OrganizationID,
		Sha1:           url.Sha1,
		Title:          input.Title,
		URL:            url.CanonicalURL,
	}

	err = link.Upsert(
		ctx,
		r.Tx,
		true,
		[]string{"organization_id", "sha1"},
		boil.Whitelist("url", "title"),
		boil.Infer(),
	)

	if err != nil {
		return nil, err
	}

	return &models.UpsertLinkPayload{
		LinkEdge: models.LinkEdge{Node: link},
	}, nil
}
