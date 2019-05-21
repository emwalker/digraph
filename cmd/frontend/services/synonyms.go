package services

import (
	"context"
	"fmt"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

var defaultSynonymSortOrder = qm.OrderBy("locale <> 'en', synonyms.created_at asc")

// AddSynonymResult holds the result of an AddSynonym call.
type AddSynonymResult struct {
	Alerts         []models.Alert
	Synonym        *models.Synonym
	SynonymCreated bool
	Cleanup        CleanupFunc
}

// DeleteSynonymResult holds the result of an UpdateTopic call.
type DeleteSynonymResult struct {
	Alerts  []models.Alert
	Cleanup CleanupFunc
	Success bool
	Topic   *models.Topic
}

func (c Connection) updateTopicNameFromSynonym(ctx context.Context, topic *models.Topic) (bool, error) {
	synonym, err := models.Synonyms(
		qm.Where("topic_id = ?", topic.ID),
		qm.Limit(1),
		defaultSynonymSortOrder,
	).One(ctx, c.Exec)

	if err != nil {
		return false, err
	}

	repo, err := topic.Repository().One(ctx, c.Exec)
	if err != nil {
		return false, err
	}

	count, err := repo.Topics(qm.Where("name = ? and id <> ?", synonym.Name, topic.ID)).Count(ctx, c.Exec)
	if count > 0 {
		return true, nil
	}

	log.Printf("Updating topic %s with name %s", topic.ID, synonym.Name)
	topic.Name = synonym.Name
	if _, err = topic.Update(ctx, c.Exec, boil.Infer()); err != nil {
		return false, err
	}

	return false, nil
}

// AddSynonym adds a synonym to a topic.
func (c Connection) AddSynonym(
	ctx context.Context, topic *models.Topic, name string, locale models.LocaleIdentifier,
) (*AddSynonymResult, error) {
	var ok bool

	name, ok = NormalizeTopicName(name)
	if !ok {
		return &AddSynonymResult{
			Alerts:  []models.Alert{*invalidNameWarning(name)},
			Cleanup: noopCleanup,
		}, nil
	}

	synonym := models.Synonym{
		Locale:  string(locale),
		Name:    name,
		TopicID: topic.ID,
	}

	log.Printf("Adding synonym to topic %s: %v", topic.ID, synonym)

	err := synonym.Insert(ctx, c.Exec, boil.Infer())
	if err != nil {
		return nil, err
	}

	cleanup := func() error {
		if _, err = synonym.Delete(ctx, c.Exec); err != nil {
			return err
		}
		return nil
	}

	if anotherTopic, err := c.updateTopicNameFromSynonym(ctx, topic); err != nil {
		if anotherTopic {
			alerts := []models.Alert{
				*models.NewAlert(models.AlertTypeWarn, "A topic with this name already exists"),
			}

			return &AddSynonymResult{
				Alerts:         alerts,
				Cleanup:        cleanup,
				Synonym:        &synonym,
				SynonymCreated: true,
			}, nil
		}
		return nil, err
	}

	return &AddSynonymResult{
		Cleanup:        cleanup,
		Synonym:        &synonym,
		SynonymCreated: true,
	}, nil
}

// DeleteSynonym deletes a synonym from a topic.
func (c Connection) DeleteSynonym(ctx context.Context, synonym *models.Synonym) (*DeleteSynonymResult, error) {
	log.Printf("Deleting synonym %s", synonym.ID)

	topic, err := models.Topics(
		qm.InnerJoin("synonyms s on s.topic_id = topics.id"),
		qm.InnerJoin("repositories r on r.id = topics.repository_id"),
		qm.InnerJoin("organization_members om on r.organization_id = om.organization_id"),
		qm.Where("om.user_id = ?", c.Actor.ID),
		qm.Where("s.id = ?", synonym.ID),
	).One(ctx, c.Exec)

	if err != nil {
		return nil, err
	}

	synonyms, err := models.Synonyms(
		qm.Where("topic_id = ?", topic.ID),
		qm.Where("name <> ?", topic.Name),
		qm.Limit(2),
		defaultSynonymSortOrder,
	).All(ctx, c.Exec)

	if err != nil {
		log.Printf("Could not count synonyms for topic %s: %s", topic.ID, err)
		return nil, err
	}

	if len(synonyms) < 1 {
		alerts := []models.Alert{
			*models.NewAlert(models.AlertTypeWarn, "Cannot delete remaining synonyms"),
		}

		return &DeleteSynonymResult{
			Alerts:  alerts,
			Cleanup: noopCleanup,
			Topic:   topic,
		}, nil
	}

	nextSynonym := synonyms[0]

	repo, err := topic.Repository().One(ctx, c.Exec)
	if err != nil {
		return nil, err
	}

	topicExists, err := repo.Topics(qm.Where("name = ?", nextSynonym.Name)).Exists(ctx, c.Exec)
	if err != nil {
		return nil, err
	}

	if topicExists {
		alerts := []models.Alert{
			*models.NewAlert(models.AlertTypeWarn,
				fmt.Sprintf("Conflicting topic prevents synonym from being deleted: %s", nextSynonym.Name),
			),
		}

		return &DeleteSynonymResult{
			Alerts:  alerts,
			Cleanup: noopCleanup,
			Topic:   topic,
		}, nil
	}

	if _, err = synonym.Delete(ctx, c.Exec); err != nil {
		log.Printf("There was a problem deleting synonym %s: %s", synonym.ID, err)
		return nil, err
	}

	topic.Name = nextSynonym.Name
	if _, err = topic.Update(ctx, c.Exec, boil.Infer()); err != nil {
		return nil, err
	}

	return &DeleteSynonymResult{
		Cleanup: noopCleanup,
		Success: true,
		Topic:   topic,
	}, nil
}
