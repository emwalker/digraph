package resolvers

import (
	"context"
	"crypto/sha1"
	"database/sql"
	"fmt"
	"log"
	"os"

	pl "github.com/PuerkitoBio/purell"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers/pageinfo"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func init() {
	log.SetOutput(os.Stdout)
}

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
func (r *MutationResolver) CreateTopic(
	ctx context.Context, input models.CreateTopicInput,
) (*models.CreateTopicPayload, error) {
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
func (r *MutationResolver) SelectTopic(
	ctx context.Context, input models.SelectTopicInput,
) (*models.SelectTopicPayload, error) {
	panic("not implemented")
}

type URL struct {
	CanonicalURL string
	Input        string
	Sha1         string
}

const normalizationFlags = pl.FlagRemoveDefaultPort |
	pl.FlagDecodeDWORDHost |
	pl.FlagDecodeOctalHost |
	pl.FlagDecodeHexHost |
	pl.FlagRemoveUnnecessaryHostDots |
	pl.FlagRemoveDotSegments |
	pl.FlagRemoveDuplicateSlashes |
	pl.FlagUppercaseEscapes |
	pl.FlagDecodeUnnecessaryEscapes |
	pl.FlagEncodeNecessaryEscapes |
	pl.FlagSortQuery

func normalizeUrl(url string) (*URL, error) {
	canonical, err := pl.NormalizeURLString(url, normalizationFlags)
	if err != nil {
		return nil, err
	}

	sha1 := fmt.Sprintf("%x", sha1.Sum([]byte(canonical)))
	return &URL{canonical, url, sha1}, nil
}

// UpdateTopic updates the fields on a topic.
func (r *MutationResolver) UpdateTopic(
	ctx context.Context, input models.UpdateTopicInput,
) (*models.UpdateTopicPayload, error) {
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

func providedOrFetchedTitle(
	fetcher pageinfo.Fetcher, url string, providedTitle *string,
) (string, error) {
	if providedTitle != nil && *providedTitle != "" {
		return *providedTitle, nil
	}

	log.Print("Fetching title of ", url)
	pageInfo, err := fetcher.FetchPage(url)
	if err != nil {
		return "", err
	}

	if pageInfo.Title != nil {
		return *pageInfo.Title, nil
	}

	return "", nil
}

func topicsFromIds(topicIds []string) []*models.Topic {
	var topics []*models.Topic
	for _, topicID := range topicIds {
		topics = append(topics, &models.Topic{ID: topicID})
	}
	return topics
}

func addParentTopicsToLink(
	ctx context.Context, tx *sql.Tx, link models.Link, parentTopicIds []string,
) error {
	if len(parentTopicIds) < 1 {
		return nil
	}

	var topicIds []interface{}
	for _, topicID := range parentTopicIds {
		topicIds = append(topicIds, topicID)
	}

	overlappingTopics, err := link.ParentTopics(
		qm.Select("id"),
		qm.WhereIn("id in ?", topicIds...),
	).All(ctx, tx)

	if err != nil {
		return err
	}

	seen := make(map[string]bool)
	for _, topic := range overlappingTopics {
		seen[topic.ID] = true
	}

	var insertIds []string
	for _, topicID := range parentTopicIds {
		if _, ok := seen[topicID]; !ok {
			insertIds = append(insertIds, topicID)
		}
	}

	if len(insertIds) < 1 {
		return nil
	}

	topics := topicsFromIds(insertIds)
	return link.AddParentTopics(ctx, tx, false, topics...)
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(
	ctx context.Context, input models.UpsertLinkInput,
) (*models.UpsertLinkPayload, error) {
	url, err := normalizeUrl(input.URL)
	if err != nil {
		return nil, err
	}

	title, err := providedOrFetchedTitle(r.Fetcher, url.CanonicalURL, input.Title)
	if err != nil {
		return nil, err
	}

	link := models.Link{
		OrganizationID: input.OrganizationID,
		Sha1:           url.Sha1,
		Title:          title,
		URL:            url.CanonicalURL,
	}

	err = transact(r.DB, func(tx *sql.Tx) error {
		err = link.Upsert(
			ctx,
			tx,
			true,
			[]string{"organization_id", "sha1"},
			boil.Whitelist("url", "title"),
			boil.Infer(),
		)

		if err != nil {
			return err
		}

		err = addParentTopicsToLink(ctx, tx, link, input.AddParentTopicIds)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return &models.UpsertLinkPayload{
		LinkEdge: models.LinkEdge{Node: link},
	}, nil
}

// UpdateItemTopics sets the topics on a link that implements the Topicable interface.
func (r *MutationResolver) UpdateLinkTopics(
	ctx context.Context, input models.UpdateLinkTopicsInput,
) (*models.UpdateLinkTopicsPayload, error) {
	link, err := models.Links(qm.Where("id = ?", input.LinkID)).One(ctx, r.DB)

	topics := topicsFromIds(input.ParentTopicIds)
	err = link.SetParentTopics(ctx, r.DB, false, topics...)
	if err != nil {
		return nil, err
	}

	err = link.Reload(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	return &models.UpdateLinkTopicsPayload{
		Link: *link,
	}, nil
}
