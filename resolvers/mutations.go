package resolvers

import (
	"context"
	"crypto/sha1"
	"database/sql"
	"fmt"
	"log"
	"net/url"
	"os"

	pl "github.com/PuerkitoBio/purell"
	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers/pageinfo"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
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

func cycleWarning(descendant, ancestor *models.Topic) *models.Alert {
	return models.NewAlert(
		models.AlertTypeWarn,
		fmt.Sprintf(
			`"%s" is a descendant of "%s" and cannot be added as a parent topic`,
			descendant.Name,
			ancestor.Name,
		),
	)
}

func invalidNameWarning(name string) *models.Alert {
	return models.NewAlert(
		models.AlertTypeWarn,
		fmt.Sprintf("Not a valid topic name: %s", name),
	)
}

func isDescendantOf(ctx context.Context, exec boil.ContextExecutor, topic, ancestor *models.Topic) (bool, error) {
	type resultInfo struct {
		Count int `boil:"match_count"`
	}

	var result resultInfo

	// TODO: do a batch query and look for matching ids in the result instead of iterating over each
	// individual topic.
	err := queries.Raw(`
	with recursive children as (
	  select parent_id, child_id from topic_topics
	  where parent_id = $1
	union
	  select pt.child_id, ct.child_id
	  from topic_topics ct
	  inner join children pt on pt.child_id = ct.parent_id
	)
	select count(*) match_count from children c where c.child_id = $2
	`, ancestor.ID, topic.ID).Bind(ctx, exec, &result)

	if err != nil {
		return false, err
	}

	return result.Count > 0, nil
}

func upsertTopic(
	ctx context.Context, exec boil.ContextExecutor, org *models.Organization, topic *models.Topic,
) (*models.Topic, bool, error) {
	existing, _ := org.Topics(qm.Where("name ilike ?", topic.Name)).One(ctx, exec)
	if existing != nil {
		log.Printf("Topic %s already exists", topic.Name)
		return existing, false, nil
	}

	log.Printf("Creating new topic %s", topic.Name)
	err := topic.Insert(ctx, exec, boil.Infer())
	if err != nil {
		return nil, false, err
	}

	return topic, true, nil
}

func parentTopicsToAdd(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic, topicIds []string,
) ([]*models.Topic, []models.Alert, error) {
	parentTopics, err := topic.ParentTopics().All(ctx, exec)
	if err != nil {
		return nil, nil, err
	}

	seen := map[string]bool{}
	for _, parent := range parentTopics {
		seen[parent.ID] = true
	}

	var parents []*models.Topic
	var alerts []models.Alert

	for _, parentId := range topicIds {
		if _, ok := seen[parentId]; ok {
			continue
		}
		parent := &models.Topic{ID: parentId}
		willHaveCycle, err := isDescendantOf(ctx, exec, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, exec)
			alerts = append(alerts, *cycleWarning(parent, topic))
		} else {
			parents = append(parents, parent)
		}
	}

	return parents, alerts, nil
}

func isURL(name string) bool {
	_, err := url.ParseRequestURI(name)
	if err != nil {
		return false
	}
	return true
}

// UpsertTopic creates a new topic.
func (r *MutationResolver) UpsertTopic(
	ctx context.Context, input models.UpsertTopicInput,
) (*models.UpsertTopicPayload, error) {
	tx, err := r.DB.Begin()
	if err != nil {
		return nil, err
	}

	if isURL(input.Name) {
		tx.Rollback()
		return &models.UpsertTopicPayload{
			Alerts: []models.Alert{*invalidNameWarning(input.Name)},
		}, nil
	}

	org, err := models.FindOrganization(ctx, tx, input.OrganizationID)
	if err != nil {
		tx.Rollback()
		return nil, err
	}

	topic := &models.Topic{
		Description:    null.StringFromPtr(input.Description),
		Name:           input.Name,
		OrganizationID: input.OrganizationID,
	}
	var created bool

	topic, created, err = upsertTopic(ctx, tx, org, topic)
	if err != nil {
		tx.Rollback()
		return nil, err
	}

	parents, alerts, err := parentTopicsToAdd(ctx, tx, topic, input.TopicIds)
	if err != nil {
		tx.Rollback()
		return nil, err
	}

	if len(alerts) > 0 {
		tx.Rollback()
		return &models.UpsertTopicPayload{Alerts: alerts}, nil
	}

	if len(parents) > 0 {
		err = topic.AddParentTopics(ctx, tx, false, parents...)
		if err != nil {
			tx.Rollback()
			return nil, err
		}
	}
	tx.Commit()

	err = topic.Reload(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	if !created {
		alerts = append(
			alerts,
			*models.NewAlert(
				models.AlertTypeSuccess,
				fmt.Sprintf("A topic with the name \"%s\" was found", input.Name),
			),
		)
	}

	return &models.UpsertTopicPayload{
		Alerts:    alerts,
		TopicEdge: &models.TopicEdge{Node: *topic},
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

	existing, err := models.Links(
		qm.Where("organization_id = ? and sha1 like ?", input.OrganizationID, url.Sha1),
	).Count(ctx, r.DB)
	if err != nil {
		return nil, err
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

	if existing < 1 {
		return &models.UpsertLinkPayload{LinkEdge: &models.LinkEdge{Node: link}}, nil
	}

	return &models.UpsertLinkPayload{
		Alerts: []models.Alert{
			*models.NewAlert(models.AlertTypeSuccess, fmt.Sprintf("An existing link %s was found", input.URL)),
		},
		LinkEdge: &models.LinkEdge{Node: link},
	}, nil
}

// UpdateLinkTopics sets the parent topics on a link.
func (r *MutationResolver) UpdateLinkTopics(
	ctx context.Context, input models.UpdateLinkTopicsInput,
) (*models.UpdateLinkTopicsPayload, error) {
	link, err := models.FindLink(ctx, r.DB, input.LinkID)
	if err != nil {
		return nil, err
	}

	topics := topicsFromIds(input.ParentTopicIds)
	if err = link.SetParentTopics(ctx, r.DB, false, topics...); err != nil {
		return nil, err
	}

	if err = link.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.UpdateLinkTopicsPayload{
		Link: *link,
	}, nil
}

func parentTopicsAndAlerts(
	ctx context.Context, tx *sql.Tx, topic *models.Topic, parentIds []string,
) ([]*models.Topic, []models.Alert, error) {
	maybeParentTopics := topicsFromIds(parentIds)
	var parentTopics []*models.Topic
	var alerts []models.Alert

	for _, parent := range maybeParentTopics {
		willHaveCycle, err := isDescendantOf(ctx, tx, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, tx)
			alerts = append(alerts, *cycleWarning(parent, topic))
		} else {
			parentTopics = append(parentTopics, parent)
		}
	}

	return parentTopics, alerts, nil
}

// UpdateTopicParentTopics sets the parent topics on a topic.
func (r *MutationResolver) UpdateTopicParentTopics(
	ctx context.Context, input models.UpdateTopicParentTopicsInput,
) (*models.UpdateTopicParentTopicsPayload, error) {
	tx, err := r.DB.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}

	topic, err := models.FindTopic(ctx, tx, input.TopicID)
	if err != nil {
		return nil, err
	}

	parentTopics, alerts, err := parentTopicsAndAlerts(ctx, tx, topic, input.ParentTopicIds)

	if err = topic.SetParentTopics(ctx, tx, false, parentTopics...); err != nil {
		return nil, err
	}

	if err = topic.Reload(ctx, tx); err != nil {
		return nil, err
	}

	if err = tx.Commit(); err != nil {
		return nil, err
	}

	return &models.UpdateTopicParentTopicsPayload{
		Alerts: alerts,
		Topic:  *topic,
	}, nil
}
