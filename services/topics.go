package services

import (
	"context"
	"fmt"
	"log"

	"github.com/emwalker/digraph/common"
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type UpdateTopicParentTopicsResult struct {
	Alerts []models.Alert
	Topic  *models.Topic
}

type UpsertTopicResult struct {
	Alerts       []models.Alert
	Topic        *models.Topic
	TopicCreated bool
}

func UpdateTopicParentTopics(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic, parentTopicIds []string,
) (*UpdateTopicParentTopicsResult, error) {
	parentTopics, alerts, err := parentTopicsAndAlerts(ctx, exec, topic, parentTopicIds)

	if err = topic.SetParentTopics(ctx, exec, false, parentTopics...); err != nil {
		return nil, err
	}

	if err = topic.Reload(ctx, exec); err != nil {
		return nil, err
	}

	return &UpdateTopicParentTopicsResult{alerts, topic}, nil
}

func UpsertTopic(
	ctx context.Context, exec boil.ContextExecutor, organizationID, name string, description *string,
	parentTopicIds []string,
) (*UpsertTopicResult, error) {
	if isURL(name) {
		return &UpsertTopicResult{
			Alerts: []models.Alert{*invalidNameWarning(name)},
		}, nil
	}

	org, err := models.FindOrganization(ctx, exec, organizationID)
	if err != nil {
		return nil, err
	}

	topic := &models.Topic{
		Description:    null.StringFromPtr(description),
		Name:           name,
		OrganizationID: organizationID,
	}
	var created bool

	topic, created, err = upsertTopic(ctx, exec, org, topic)
	if err != nil {
		return nil, err
	}

	parents, alerts, err := parentTopicsToAdd(ctx, exec, topic, parentTopicIds)
	if err != nil {
		return nil, err
	}

	if len(alerts) > 0 {
		return &UpsertTopicResult{Alerts: alerts}, nil
	}

	if len(parents) > 0 {
		err = topic.AddParentTopics(ctx, exec, false, parents...)
		if err != nil {
			return nil, err
		}
	}

	err = topic.Reload(ctx, exec)
	if err != nil {
		return nil, err
	}

	if !created {
		alerts = append(
			alerts,
			*models.NewAlert(
				models.AlertTypeSuccess,
				fmt.Sprintf("A topic with the name \"%s\" was found", name),
			),
		)
	}

	return &UpsertTopicResult{
		Alerts:       alerts,
		Topic:        topic,
		TopicCreated: created,
	}, nil
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

func parentTopicsAndAlerts(
	ctx context.Context, exec boil.ContextExecutor, topic *models.Topic, parentIds []string,
) ([]*models.Topic, []models.Alert, error) {
	maybeParentTopics := common.TopicsFromIds(parentIds)
	var parentTopics []*models.Topic
	var alerts []models.Alert

	for _, parent := range maybeParentTopics {
		willHaveCycle, err := isDescendantOf(ctx, exec, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, exec)
			alerts = append(alerts, *cycleWarning(parent, topic))
		} else {
			parentTopics = append(parentTopics, parent)
		}
	}

	return parentTopics, alerts, nil
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
