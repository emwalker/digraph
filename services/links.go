package services

import (
	"context"
	"crypto/sha1"
	"fmt"
	"log"
	"net/url"

	pl "github.com/PuerkitoBio/purell"
	"github.com/emwalker/digraph/common"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/services/pageinfo"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type UpsertLinkResult struct {
	Alerts      []models.Alert
	Link        models.Link
	LinkCreated bool
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

var Fetcher pageinfo.Fetcher = &pageinfo.HtmlFetcher{}

func NormalizeUrl(url string) (*URL, error) {
	canonical, err := pl.NormalizeURLString(url, normalizationFlags)
	if err != nil {
		return nil, err
	}

	sha1 := fmt.Sprintf("%x", sha1.Sum([]byte(canonical)))
	return &URL{canonical, url, sha1}, nil
}

func providedOrFetchedTitle(url string, providedTitle *string) (string, error) {
	if providedTitle != nil && *providedTitle != "" {
		return *providedTitle, nil
	}

	log.Print("Fetching title of ", url)
	pageInfo, err := Fetcher.FetchPage(url)
	if err != nil {
		return "", err
	}

	if pageInfo.Title != nil {
		return *pageInfo.Title, nil
	}

	return "", nil
}

func isURL(name string) bool {
	_, err := url.ParseRequestURI(name)
	if err != nil {
		return false
	}
	return true
}

func addParentTopicsToLink(
	ctx context.Context, exec boil.ContextExecutor, link models.Link, parentTopicIds []string,
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
	).All(ctx, exec)

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

	topics := common.TopicsFromIds(insertIds)
	return link.AddParentTopics(ctx, exec, false, topics...)
}

func UpsertLink(
	ctx context.Context, exec boil.ContextExecutor, organizationID, providedUrl string,
	providedTitle *string, parentTopicIds []string,
) (*UpsertLinkResult, error) {
	var alerts []models.Alert

	url, err := NormalizeUrl(providedUrl)
	if err != nil {
		return nil, err
	}

	title, err := providedOrFetchedTitle(url.CanonicalURL, providedTitle)
	if err != nil {
		return nil, err
	}

	link := models.Link{
		OrganizationID: organizationID,
		Sha1:           url.Sha1,
		Title:          title,
		URL:            url.CanonicalURL,
	}

	existing, err := models.Links(
		qm.Where("organization_id = ? and sha1 like ?", organizationID, url.Sha1),
	).Count(ctx, exec)
	if err != nil {
		return nil, err
	}

	err = link.Upsert(
		ctx, exec, true, []string{"organization_id", "sha1"}, boil.Whitelist("url", "title"), boil.Infer(),
	)

	if err != nil {
		return nil, err
	}

	err = addParentTopicsToLink(ctx, exec, link, parentTopicIds)
	if err != nil {
		return nil, err
	}

	if existing > 0 {
		alerts = []models.Alert{
			*models.NewAlert(models.AlertTypeSuccess, fmt.Sprintf("An existing link %s was found", providedUrl)),
		}
	}

	return &UpsertLinkResult{
		Alerts:      alerts,
		Link:        link,
		LinkCreated: existing < 1,
	}, nil
}
