package services

import (
	"context"
	"encoding/json"
	coreerrors "errors"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/emwalker/digraph/golang/internal/models"
	dqueries "github.com/emwalker/digraph/golang/internal/queries"
	"github.com/emwalker/digraph/golang/internal/services/pageinfo"
	"github.com/emwalker/digraph/golang/internal/text"
	"github.com/emwalker/digraph/golang/internal/util"
	"github.com/pkg/errors"
	"github.com/volatiletech/null/v8"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

var (
	errCannotDeleteRootTopic = coreerrors.New("cannot delete root topic")
	errInvalidSynonym        = coreerrors.New("invalid synonym")
)

// DeleteTopicTimeRangeResult holds the result of an attempt to delete the time range on a topic.
type DeleteTopicTimeRangeResult struct {
	Alerts             []*models.Alert
	Topic              *models.Topic
	DeletedTimeRangeID *string
}

// UpdateSynonymsResult holds the result after updating the synonyms on a topic.
type UpdateSynonymsResult struct {
	Alerts []*models.Alert
	Topic  *models.Topic
}

// UpdateTopicResult holds the result of an UpdateTopic call.
type UpdateTopicResult struct {
	Alerts []*models.Alert
	Topic  *models.Topic
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

func nameAlreadyExists(name string) *models.Alert {
	return models.NewAlert(
		models.AlertTypeWarn,
		fmt.Sprintf("There is already a topic with the name '%s' in the same repository", name),
	)
}

// NormalizeTopicName returns a normalized topic name and indicates whether the name is valid.
func NormalizeTopicName(name string) (string, bool) {
	name = text.Squash(name)

	if name == "" {
		return "", false
	}

	if pageinfo.IsURL(name) {
		return name, false
	}

	return name, true
}

// DisplayName constructs a display name from a SynonymList and a Timerange.
func DisplayName(
	timerange *models.Timerange, synonyms *models.SynonymList, locale models.LocaleIdentifier,
) (string, error) {
	name, ok := synonyms.NameForLocale(locale)
	if !ok {
		return "<name missing>", coreerrors.New("name not found")
	}

	if timerange == nil {
		return name, nil
	}

	if timerange.StartsAt.IsZero() {
		return name, coreerrors.New("startsAt is not valid")
	}

	startsAt := timerange.StartsAt

	switch models.TimeRangePrefixFormat(timerange.PrefixFormat) {
	case models.TimeRangePrefixFormatStartYear:
		return fmt.Sprintf("%s %s", startsAt.Format("2006"), name), nil
	case models.TimeRangePrefixFormatStartYearMonth:
		return fmt.Sprintf("%s %s", startsAt.Format("2006-01"), name), nil
	}

	return name, nil
}

// DeleteTopic deletes a topic and associated relations.
type DeleteTopic struct {
	Actor *models.User
	Topic *models.Topic
}

// DeleteTopicResult holds the result of an attempt to delete a topic.
type DeleteTopicResult struct {
	Alerts         []*models.Alert
	DeletedTopicID *string
}

// Call removes a time range from a topic.
func (m DeleteTopic) Call(ctx context.Context, exec boil.ContextExecutor) (*DeleteTopicResult, error) {
	topic := m.Topic

	if topic.Root {
		log.Printf("%s attempted to delete root %s", m.Actor, topic)
		return nil, errCannotDeleteRootTopic
	}

	log.Printf("%s is deleting %s; moving its topics and links to its parent topics", m.Actor, topic)

	parentTopics, err := topic.ParentTopics().All(ctx, exec)
	if err != nil {
		return nil, errors.Wrap(err, "services: failed to fetch parent topics")
	}

	childTopics, err := topic.ChildTopics().All(ctx, exec)
	if dqueries.IsRealError(err) {
		return nil, errors.Wrap(err, "services: failed to fetch child topics")
	}

	links, err := topic.ChildLinks().All(ctx, exec)
	if dqueries.IsRealError(err) {
		return nil, errors.Wrap(err, "services: failed to fetch child links")
	}

	for _, parentTopic := range parentTopics {
		// Add any child topics to the parent topics of the topic to be deleted.
		for _, childTopic := range childTopics {
			values := []interface{}{parentTopic.ID, childTopic.ID}
			_, err = exec.ExecContext(ctx, "select add_topic_to_topic($1, $2)", values...)
			if err != nil {
				return nil, errors.Wrap(err, "services: failed to re-seat child topics")
			}
		}

		// Add any child links to the parent topics of the topic to be deleted.
		for _, link := range links {
			values := []interface{}{parentTopic.ID, link.ID}
			_, err = exec.ExecContext(ctx, "select add_topic_to_link($1, $2)", values...)
			if err != nil {
				return nil, errors.Wrap(err, "services: failed to re-seat child links")
			}
		}
	}

	if _, err = topic.Delete(ctx, exec); err != nil {
		return nil, errors.Wrap(err, "services: failed to delete topic")
	}

	return &DeleteTopicResult{
		DeletedTopicID: &topic.ID,
	}, nil
}

// DeleteTopicTimeRange removes a time range from a topic.
func (c Connection) DeleteTopicTimeRange(
	ctx context.Context, topic *models.Topic,
) (*DeleteTopicTimeRangeResult, error) {
	timerange, err := dqueries.TimeRange(ctx, c.Exec, topic)
	if err != nil && err.Error() != dqueries.ErrSQLNoRows {
		return nil, errors.Wrap(err, "services: failed to fetch time range")
	}

	var timerangeID *string
	if timerange != nil {
		timerangeID = &timerange.ID
		if _, err = timerange.Delete(ctx, c.Exec); err != nil {
			return nil, errors.Wrap(err, "services: failed to delete time range")
		}
	}

	topic.TimerangeID = null.StringFromPtr(nil)
	if err = c.updateTopicName(ctx, topic, nil); err != nil {
		return nil, errors.Wrap(err, "services: failed to update topic name")
	}

	if err = topic.Reload(ctx, c.Exec); err != nil {
		return nil, errors.Wrap(err, "services: failed to reload topic")
	}

	return &DeleteTopicTimeRangeResult{
		DeletedTimeRangeID: timerangeID,
		Topic:              topic,
	}, nil
}

// UpdateSynonyms updates the synonyms of the topic.
func (c Connection) UpdateSynonyms(
	ctx context.Context, topic *models.Topic, synonyms []models.Synonym,
) (*UpdateSynonymsResult, error) {
	log.Printf("Updating synonyms for topic %s", topic)

	seen := make(map[models.Synonym]bool)
	dedupedSynonyms := []models.Synonym{}

	for _, synonym := range synonyms {
		normalized := synonym
		normalizedName, validName := NormalizeTopicName(synonym.Name)

		if validName {
			normalized.Name = normalizedName
		} else {
			log.Printf("Not a valid synonym: %s", synonym)
			return nil, errors.Wrap(errInvalidSynonym, "services: failed to update synonyms")
		}

		if _, ok := seen[normalized]; !ok {
			seen[normalized] = true
			dedupedSynonyms = append(dedupedSynonyms, normalized)
		}
	}

	if err := topic.Synonyms.Marshal(&dedupedSynonyms); err != nil {
		return nil, err
	}

	timerange, err := dqueries.TimeRange(ctx, c.Exec, topic)
	if err != nil {
		return nil, errors.Wrap(err, "services: failed to fetch time range")
	}

	topic.Name, err = DisplayName(timerange, &models.SynonymList{Values: dedupedSynonyms}, models.LocaleIdentifierEn)
	if err != nil {
		return nil, errors.Wrap(err, "services: failed to update display name")
	}

	if _, err := topic.Update(ctx, c.Exec, boil.Whitelist("synonyms", "name")); err != nil {
		log.Printf("Unable to update synonyms for topic %s", topic)
		return nil, err
	}

	return &UpdateSynonymsResult{}, nil
}

// UpdateTopic updates fields on the topic.
func (c Connection) UpdateTopic(
	ctx context.Context, topic *models.Topic, name string, description *string,
) (*UpdateTopicResult, error) {
	topic.Name = name
	topic.Description = null.StringFromPtr(description)

	if _, err := topic.Update(ctx, c.Exec, boil.Infer()); err != nil {
		// Does sqlboiler provide a way to get to the pq error?
		if strings.Contains(err.Error(), "pq: duplicate key value violates unique constraint") {
			return &UpdateTopicResult{
				Alerts: []*models.Alert{nameAlreadyExists(name)},
			}, nil
		}
		log.Printf("services.UpdateTopic: %s (type %T)", err, err)
		return nil, fmt.Errorf("there was a problem updating topic %s", topic.ID)
	}

	return &UpdateTopicResult{Topic: topic}, nil
}

// UpsertTopicResult holds the result of an UpsertTopic call.
type UpsertTopicResult struct {
	Alerts       []*models.Alert
	Topic        *models.Topic
	TopicCreated bool
}

// UpsertTopic updates a topic if it exists and creates it if it does not exist.
func (c Connection) UpsertTopic(
	ctx context.Context, repo *models.Repository, name string, description *string,
	parentTopicIds []string,
) (*UpsertTopicResult, error) {
	name, ok := NormalizeTopicName(name)

	if !ok {
		return &UpsertTopicResult{
			Alerts: []*models.Alert{invalidNameWarning(name)},
		}, nil
	}

	synonyms := models.SynonymList{
		Values: []models.Synonym{
			{Locale: "en", Name: name},
		},
	}

	synonymJSON, err := json.Marshal(&synonyms.Values)
	if err != nil {
		log.Printf("Unable to encode json for synonym: %s", name)
		return nil, err
	}

	topic := &models.Topic{
		Description:    null.StringFromPtr(description),
		Name:           name,
		RepositoryID:   repo.ID,
		Synonyms:       synonymJSON,
		OrganizationID: repo.OrganizationID,
	}
	var created bool

	if topic, created, err = c.upsertTopic(ctx, repo, topic); err != nil {
		return nil, fmt.Errorf("upsertTopic: %s", err)
	}

	if len(parentTopicIds) < 1 {
		var rootTopic *models.Topic
		if rootTopic, err = repo.Topics(qm.Where("root")).One(ctx, c.Exec); err != nil {
			return nil, fmt.Errorf("upsertTopic: %s", err)
		}
		parentTopicIds = append(parentTopicIds, rootTopic.ID)
	}

	parents, alerts, err := c.parentTopicsToAdd(ctx, topic, parentTopicIds)
	if err != nil {
		return nil, err
	}

	if len(alerts) > 0 {
		return &UpsertTopicResult{Alerts: alerts}, nil
	}

	for _, parentTopic := range parents {
		values := []interface{}{parentTopic.ID, topic.ID}
		_, err = c.Exec.ExecContext(ctx, "select add_topic_to_topic($1, $2)", values...)
		if err != nil {
			return nil, fmt.Errorf("UpsertTopic: %s", err)
		}
	}

	err = topic.Reload(ctx, c.Exec)
	if err != nil {
		return nil, err
	}

	if !created {
		alerts = append(
			alerts,
			models.NewAlert(
				models.AlertTypeSuccess,
				fmt.Sprintf(`A topic with the name "%s" was found`, name),
			),
		)
	}

	return &UpsertTopicResult{
		Alerts:       alerts,
		Topic:        topic,
		TopicCreated: created,
	}, nil
}

func isDescendantOf(
	ctx context.Context, exec boil.ContextExecutor, topic, ancestor *models.Topic,
) (bool, error) {
	type resultInfo struct {
		Count int `boil:"match_count"`
	}

	var result resultInfo

	err := queries.Raw(`
	select count(*) match_count
	from topic_down_set($1) tds
	where tds.child_id = $2
	`, ancestor.ID, topic.ID).Bind(ctx, exec, &result)

	if err != nil {
		return false, err
	}

	return result.Count > 0, nil
}

func (c Connection) upsertTopic(
	ctx context.Context, repo *models.Repository, topic *models.Topic,
) (*models.Topic, bool, error) {
	existing, err := repo.Topics(
		qm.Where("topics.name like ?", topic.Name),
	).One(ctx, c.Exec)

	if dqueries.IsRealError(err) {
		return nil, false, fmt.Errorf("upsertTopic: %s", err)
	}

	if existing != nil {
		log.Printf("Topic %s already exists", topic.Name)
		return existing, false, nil
	}

	log.Printf("Creating new topic %s", topic.Name)

	synonyms := []models.Synonym{
		{Name: topic.Name, Locale: "en"},
	}

	jsonArray, err := json.Marshal(synonyms)
	if err != nil {
		log.Printf("Unable to marshal json for %v: %s", synonyms, err)
		return topic, false, err
	}

	topic.Synonyms = jsonArray

	err = topic.Insert(ctx, c.Exec, boil.Infer())
	if err != nil {
		log.Printf("Failed to insert topic %v: %s", topic, err)
		return nil, false, err
	}

	return topic, true, nil
}

func (c Connection) parentTopicsToAdd(
	ctx context.Context, topic *models.Topic, topicIds []string,
) ([]*models.Topic, []*models.Alert, error) {
	parentTopics, err := topic.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		return nil, nil, err
	}

	seen := map[string]bool{}
	for _, parent := range parentTopics {
		seen[parent.ID] = true
	}

	var parents []*models.Topic
	var alerts []*models.Alert

	for _, parentID := range topicIds {
		if _, ok := seen[parentID]; ok {
			continue
		}
		parent := &models.Topic{ID: parentID}
		willHaveCycle, err := isDescendantOf(ctx, c.Exec, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, c.Exec)
			alerts = append(alerts, cycleWarning(parent, topic))
		} else {
			parents = append(parents, parent)
		}
	}

	return parents, alerts, nil
}

func (c Connection) updateTopicName(
	ctx context.Context, topic *models.Topic, timerange *models.Timerange,
) error {
	synonyms, err := topic.SynonymList()
	if err != nil {
		return errors.Wrap(err, "services: failed to fetch synonym list")
	}

	// Updating of the topic name is temporary; eventually the topic name will go away, as it is
	// necessarily locale-specific, and we want all locales to work the same way. Currently the
	// topic name is used for sorting topic results at the database query.

	displayName, err := DisplayName(timerange, synonyms, models.LocaleIdentifierEn)
	if err != nil {
		return errors.Wrap(err, "services: failed to calculate display name")
	}

	topic.Name = displayName
	if _, err = topic.Update(ctx, c.Exec, boil.Infer()); err != nil {
		return errors.Wrap(err, "services: failed to update topic name")
	}

	return nil
}

// UpsertTopicTimeRangeResult returns the result of an upsert call.
type UpsertTopicTimeRangeResult struct {
	Alerts    []*models.Alert
	Topic     *models.Topic
	TimeRange *models.Timerange
}

// UpsertTopicTimeRange adds a timeline to a topic.
func (c Connection) UpsertTopicTimeRange(
	ctx context.Context, topic *models.Topic, startsAt time.Time, endsAt *time.Time, format models.TimeRangePrefixFormat,
) (*UpsertTopicTimeRangeResult, error) {
	var alerts []*models.Alert

	timerange, err := topic.Timerange().One(ctx, c.Exec)

	if err == nil {
		log.Printf("Time range already exists, updating")
		timerange.StartsAt = startsAt
		timerange.PrefixFormat = string(format)

		if _, err = timerange.Update(ctx, c.Exec, boil.Infer()); err != nil {
			return nil, errors.Wrap(err, "services: failed to update existing time range")
		}
	} else {
		if err.Error() != dqueries.ErrSQLNoRows {
			return nil, errors.Wrap(err, "services: failed to query for time range")
		}

		log.Printf("Time range does not yet exist, creating")
		timerange = &models.Timerange{
			StartsAt:     startsAt,
			PrefixFormat: string(format),
		}

		if err = timerange.Insert(ctx, c.Exec, boil.Infer()); err != nil {
			return nil, errors.Wrap(err, "services: failed to insert new time range")
		}

		topic.TimerangeID = null.NewString(timerange.ID, true)
		if _, err = topic.Update(ctx, c.Exec, boil.Whitelist("timerange_id")); err != nil {
			return nil, errors.Wrap(err, "services: failed to add time range to topic")
		}
	}

	if err = c.updateTopicName(ctx, topic, timerange); err != nil {
		log.Printf("services: failed to update name for %s", topic)
		return nil, errors.Wrap(err, "services: failed to update name")
	}

	return &UpsertTopicTimeRangeResult{
		Alerts:    alerts,
		Topic:     topic,
		TimeRange: timerange,
	}, nil
}

// UpdateTopicParentTopics updates the parent topics of a topic
type UpdateTopicParentTopics struct {
	Actor          *models.User
	Topic          *models.TopicValue
	ParentTopicIds []string
}

// UpdateTopicParentTopicsResult holds the result of an UpdateTopicParentTopics call.
type UpdateTopicParentTopicsResult struct {
	Alerts []*models.Alert
	Topic  *models.TopicValue
}

func (m UpdateTopicParentTopics) parentTopicsAndAlerts(
	ctx context.Context, exec boil.ContextExecutor,
) ([]*models.Topic, []*models.Alert, error) {
	maybeParentTopics := util.TopicsFromIds(m.ParentTopicIds)
	topic := m.Topic.Topic
	var parentTopics []*models.Topic
	var alerts []*models.Alert

	for _, parent := range maybeParentTopics {
		willHaveCycle, err := isDescendantOf(ctx, exec, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, exec)
			alerts = append(alerts, cycleWarning(parent, topic))
		} else {
			parentTopics = append(parentTopics, parent)
		}
	}

	return parentTopics, alerts, nil
}

func (m UpdateTopicParentTopics) cleanUpParentTopicLinks(
	ctx context.Context, exec boil.ContextExecutor, parentTopicsBefore, parentTopicsAfter []*models.Topic,
) error {
	removedParentTopics := make(map[string]bool)
	for _, topic := range parentTopicsBefore {
		removedParentTopics[topic.ID] = true
	}

	for _, topic := range parentTopicsAfter {
		removedParentTopics[topic.ID] = false
	}

	for topicID, removed := range removedParentTopics {
		if !removed {
			continue
		}

		for _, topic := range parentTopicsBefore {
			if topic.ID != topicID {
				continue
			}

			values := []interface{}{topic.ID}
			sql := "delete from link_transitive_closure where parent_id = $1"
			if _, err := exec.ExecContext(ctx, sql, values...); err != nil {
				return err
			}

			sql = "select upsert_link_down_set($1)"
			if _, err := exec.ExecContext(ctx, sql, values...); err != nil {
				return err
			}
		}
	}

	return nil
}

// Call updates the parent topics of a topic.
func (m UpdateTopicParentTopics) Call(ctx context.Context, exec boil.ContextExecutor) (*UpdateTopicParentTopicsResult, error) {
	topic := m.Topic
	parentTopics, alerts, err := m.parentTopicsAndAlerts(ctx, exec)
	if err != nil {
		return nil, err
	}

	originalParentTopics, err := topic.ParentTopics().All(ctx, exec)
	if err != nil {
		return nil, err
	}

	values := []interface{}{topic.ID}
	_, err = exec.ExecContext(ctx, "delete from topic_transitive_closure where child_id = $1", values...)
	if err != nil {
		return nil, err
	}

	_, err = exec.ExecContext(ctx, "delete from topic_topics where child_id = $1", values...)
	if err != nil {
		return nil, err
	}

	err = m.cleanUpParentTopicLinks(ctx, exec, originalParentTopics, parentTopics)
	if err != nil {
		return nil, err
	}

	for _, parent := range parentTopics {
		values := []interface{}{parent.ID, topic.ID}
		_, err := exec.ExecContext(ctx, "select add_topic_to_topic($1, $2)", values...)
		if err != nil {
			return nil, errors.Wrap(err, "failed to upsert topic")
		}
	}

	if err = topic.Reload(ctx, exec); err != nil {
		return nil, err
	}

	return &UpdateTopicParentTopicsResult{alerts, topic}, nil
}
