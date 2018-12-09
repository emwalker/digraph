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

func (c Connection) UpdateTopicParentTopics(
	ctx context.Context, topic *models.Topic, parentTopicIds []string,
) (*UpdateTopicParentTopicsResult, error) {
	parentTopics, alerts, err := c.parentTopicsAndAlerts(ctx, topic, parentTopicIds)

	if err = topic.SetParentTopics(ctx, c.Exec, false, parentTopics...); err != nil {
		return nil, err
	}

	if err = topic.Reload(ctx, c.Exec); err != nil {
		return nil, err
	}

	return &UpdateTopicParentTopicsResult{alerts, topic}, nil
}

func (c Connection) UpsertTopic(
	ctx context.Context, repo *models.Repository, name string, description *string,
	parentTopicIds []string,
) (*UpsertTopicResult, error) {
	if isURL(name) {
		return &UpsertTopicResult{
			Alerts: []models.Alert{*invalidNameWarning(name)},
		}, nil
	}

	topic := &models.Topic{
		Description:    null.StringFromPtr(description),
		Name:           name,
		RepositoryID:   repo.ID,
		OrganizationID: repo.OrganizationID,
	}
	var created bool
	var err error

	if topic, created, err = c.upsertTopic(ctx, repo, topic); err != nil {
		return nil, err
	}

	parents, alerts, err := c.parentTopicsToAdd(ctx, topic, parentTopicIds)
	if err != nil {
		return nil, err
	}

	if len(alerts) > 0 {
		return &UpsertTopicResult{Alerts: alerts}, nil
	}

	if len(parents) > 0 {
		err = topic.AddParentTopics(ctx, c.Exec, false, parents...)
		if err != nil {
			return nil, err
		}
	}

	err = topic.Reload(ctx, c.Exec)
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

func (c Connection) isDescendantOf(
	ctx context.Context, topic, ancestor *models.Topic,
) (bool, error) {
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
	`, ancestor.ID, topic.ID).Bind(ctx, c.Exec, &result)

	if err != nil {
		return false, err
	}

	return result.Count > 0, nil
}

func (c Connection) parentTopicsAndAlerts(
	ctx context.Context, topic *models.Topic, parentIds []string,
) ([]*models.Topic, []models.Alert, error) {
	maybeParentTopics := common.TopicsFromIds(parentIds)
	var parentTopics []*models.Topic
	var alerts []models.Alert

	for _, parent := range maybeParentTopics {
		willHaveCycle, err := c.isDescendantOf(ctx, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, c.Exec)
			alerts = append(alerts, *cycleWarning(parent, topic))
		} else {
			parentTopics = append(parentTopics, parent)
		}
	}

	return parentTopics, alerts, nil
}

func (c Connection) upsertTopic(
	ctx context.Context, repo *models.Repository, topic *models.Topic,
) (*models.Topic, bool, error) {
	existing, _ := repo.Topics(qm.Where("name ilike ?", topic.Name)).One(ctx, c.Exec)
	if existing != nil {
		log.Printf("Topic %s already exists", topic.Name)
		return existing, false, nil
	}

	log.Printf("Creating new topic %s", topic.Name)
	err := topic.Insert(ctx, c.Exec, boil.Infer())
	if err != nil {
		return nil, false, err
	}

	return topic, true, nil
}

func (c Connection) parentTopicsToAdd(
	ctx context.Context, topic *models.Topic, topicIds []string,
) ([]*models.Topic, []models.Alert, error) {
	parentTopics, err := topic.ParentTopics().All(ctx, c.Exec)
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
		willHaveCycle, err := c.isDescendantOf(ctx, parent, topic)
		if err != nil {
			return nil, nil, err
		}

		if willHaveCycle {
			parent.Reload(ctx, c.Exec)
			alerts = append(alerts, *cycleWarning(parent, topic))
		} else {
			parents = append(parents, parent)
		}
	}

	return parents, alerts, nil
}
