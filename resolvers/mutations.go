package resolvers

import (
	"context"
	"crypto/sha1"
	"database/sql"
	"fmt"

	pl "github.com/PuerkitoBio/purell"
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
)

// https://stackoverflow.com/a/23502629/61048
func transact(db *sql.DB, txFunc func(*sql.Tx) error) (err error) {
	tx, err := db.Begin()
	if err != nil {
		return
	}
	defer func() {
		if p := recover(); p != nil {
			tx.Rollback()
			panic(p)
		} else if err != nil {
			tx.Rollback()
		} else {
			err = tx.Commit()
		}
	}()
	err = txFunc(tx)
	return
}

// CreateTopic creates a new topic.
func (r *MutationResolver) CreateTopic(ctx context.Context, input models.CreateTopicInput) (*models.CreateTopicPayload, error) {
	topic := models.Topic{
		Description:    null.StringFromPtr(input.Description),
		Name:           input.Name,
		OrganizationID: input.OrganizationID,
	}

	err := transact(r.DB, func(tx *sql.Tx) error {
		err := topic.Insert(ctx, tx, boil.Infer())
		if err != nil {
			return err
		}

		var parents []*models.Topic
		for _, parentId := range input.TopicIds {
			parents = append(parents, &models.Topic{ID: parentId})
		}

		err = topic.AddParentTopics(ctx, tx, false, parents...)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return &models.CreateTopicPayload{
		TopicEdge: models.TopicEdge{Node: topic},
	}, nil
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

// UpdateTopic updates the fields on a topic.
func (r *MutationResolver) UpdateTopic(ctx context.Context, input models.UpdateTopicInput) (*models.UpdateTopicPayload, error) {
	topic := models.Topic{
		OrganizationID: input.OrganizationID,
		Name:           input.Name,
		Description:    null.StringFromPtr(input.Description),
		ID:             input.ID,
	}

	_, err := topic.Update(ctx, r.DB, boil.Infer())
	if err != nil {
		return nil, err
	}

	return &models.UpdateTopicPayload{Topic: topic}, nil
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(ctx context.Context, input models.UpsertLinkInput) (*models.UpsertLinkPayload, error) {
	url, err := normalizeUrl(input.URL)
	if err != nil {
		return nil, err
	}

	link := models.Link{
		OrganizationID: input.OrganizationID,
		Sha1:           url.Sha1,
		Title:          input.Title,
		URL:            url.CanonicalURL,
	}

	err = link.Upsert(
		ctx,
		r.DB,
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
