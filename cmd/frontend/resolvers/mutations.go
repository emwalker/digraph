package resolvers

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"log"
	"os"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/emwalker/digraph/cmd/frontend/util"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// MutationResolver holds configuration information for a mutation.
type MutationResolver struct {
	*Resolver
}

var (
	errNoAnonymousMutations = errors.New("anonymous users cannot make updates or deletions")
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

func findRepo(
	ctx context.Context, exec boil.ContextExecutor, actor *models.User, orgLogin, repoName string,
) (*models.Repository, error) {
	log.Printf("Looking for repo %s/%s as user %v", orgLogin, repoName, actor)
	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.InnerJoin("organization_members om on o.id = om.organization_id"),
		qm.Where(`
			o.login = ? and repositories.name = ? and om.user_id = ?
		`, orgLogin, repoName, actor.ID),
	}
	return models.Repositories(mods...).One(ctx, exec)
}

// DeleteLink sets the parent topics on a topic.
func (r *MutationResolver) DeleteLink(
	ctx context.Context, input models.DeleteLinkInput,
) (*models.DeleteLinkPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	link, err := models.Links(
		qm.InnerJoin("organization_members om on links.organization_id = om.organization_id"),
		qm.Where("links.id = ? and om.user_id = ?", input.LinkID, actor.ID),
		qm.Load("Repository"),
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("There was a problem looking up link: %s", input.LinkID)
		return nil, err
	}
	repo := link.R.Repository

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: actor}

		_, err := c.DeleteLink(ctx, repo, link)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed to delete link %s from repo %s: %s", actor.Summary(), link.Summary(),
			repo.Summary(), err,
		)
	}

	return &models.DeleteLinkPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedLinkID:    input.LinkID,
	}, nil
}

// DeleteTopic deletes a topic.
func (r *MutationResolver) DeleteTopic(
	ctx context.Context, input models.DeleteTopicInput,
) (*models.DeleteTopicPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	topic, err := fetchTopic(ctx, r.DB, input.TopicID, actor)
	if err != nil {
		log.Printf("There was a problem looking up topic: %s", input.TopicID)
		return nil, err
	}

	if _, err = topic.Delete(ctx, r.DB); err != nil {
		log.Printf("There was a problem deleting topic: %#v", topic)
		return nil, err
	}

	return &models.DeleteTopicPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedTopicID:   input.TopicID,
	}, nil
}

// ReviewLink marks a link reviewed.
func (r *MutationResolver) ReviewLink(
	ctx context.Context, input models.ReviewLinkInput,
) (*models.ReviewLinkPayload, error) {
	log.Printf("Adding review to link %s", input.LinkID)
	actor := GetRequestContext(ctx).Viewer()

	c := services.Connection{Exec: r.DB, Actor: actor}

	link, err := models.Links(
		qm.InnerJoin("repositories r on r.id = links.repository_id"),
		qm.InnerJoin("organization_members om on om.organization_id = r.organization_id"),
		qm.Where("links.id = ?", input.LinkID),
		qm.Where("om.user_id = ?", actor.ID),
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("Did not find link %s in the repos visible to %s: %s", input.LinkID, actor.Summary(), err)
		return nil, err
	}

	result, err := c.ReviewLink(ctx, link, input.Reviewed)
	if err != nil {
		return nil, err
	}

	return &models.ReviewLinkPayload{
		Link: &models.LinkValue{result.Link, false, actor.DefaultView()},
	}, nil
}

// SelectRepository selects the repository for the current user.
func (r *MutationResolver) SelectRepository(
	ctx context.Context, input models.SelectRepositoryInput,
) (*models.SelectRepositoryPayload, error) {
	repoID := input.RepositoryID
	actor := GetRequestContext(ctx).Viewer()
	log.Printf("Atempting to select repository %v for %#v", repoID, actor)

	var err error
	var repo *models.Repository

	if repoID == nil {
		exists, err := actor.SelectedRepository().Exists(ctx, r.DB)
		if exists {
			log.Printf("Unselecting repository from %s", actor.ID)
			repo, err = actor.SelectedRepository().One(ctx, r.DB)

			if err = actor.RemoveSelectedRepository(ctx, r.DB, repo); err != nil {
				return nil, err
			}

			if err = actor.Reload(ctx, r.DB); err != nil {
				return nil, err
			}
		}
		return &models.SelectRepositoryPayload{nil, actor}, nil
	}

	repo = &models.Repository{ID: *repoID}
	log.Printf("Selecting repository %s for user %s", repo.ID, actor.ID)
	if err = actor.SetSelectedRepository(ctx, r.DB, false, repo); err != nil {
		return nil, err
	}

	log.Printf("Reloading repo %s", repo.ID)
	if err = repo.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	log.Printf("Reloading user %s", actor.ID)
	if err = actor.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.SelectRepositoryPayload{repo, actor}, nil
}

// UpsertTopic creates a new topic.
func (r *MutationResolver) UpsertTopic(
	ctx context.Context, input models.UpsertTopicInput,
) (*models.UpsertTopicPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	var result *services.UpsertTopicResult
	var err error
	var repo *models.Repository

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err = findRepo(ctx, tx, actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			return err
		}

		c := services.Connection{Exec: tx, Actor: actor}
		result, err = c.UpsertTopic(
			ctx,
			repo,
			input.Name,
			input.Description,
			input.TopicIds,
		)

		return err
	})

	if err != nil {
		return nil, fmt.Errorf("UpsertTopic: %s", err)
	}

	if result.Topic == nil {
		return &models.UpsertTopicPayload{Alerts: result.Alerts}, nil
	}

	return &models.UpsertTopicPayload{
		Alerts: result.Alerts,
		TopicEdge: &models.TopicEdge{
			Node: &models.TopicValue{result.Topic, result.TopicCreated, actor.DefaultView()},
		},
	}, nil
}

// UpdateSynonyms updates the synonyms for a topic.
func (r *MutationResolver) UpdateSynonyms(
	ctx context.Context, input models.UpdateSynonymsInput,
) (*models.UpdateSynonymsPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	var result *services.UpdateSynonymsResult
	var err error

	topic, err := fetchTopic(ctx, r.DB, input.TopicID, actor)
	if err != nil {
		return nil, err
	}

	synonyms := make([]models.Synonym, len(input.Synonyms))

	for i, synonym := range input.Synonyms {
		locale := models.LocaleIdentifier(synonym.Locale)
		if !locale.IsValid() {
			return nil, fmt.Errorf("not a valid locale: %s", synonym.Locale)
		}

		synonyms[i] = models.Synonym{
			Locale: synonym.Locale,
			Name:   synonym.Name,
		}
	}

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: actor}

		result, err = c.UpdateSynonyms(ctx, topic, synonyms)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed update synonyms (%v) topic %s: %s", actor.Summary(), synonyms, topic.ID, err,
		)
	}

	return &models.UpdateSynonymsPayload{
		Alerts: result.Alerts,
		Topic:  &models.TopicValue{Topic: topic, View: actor.DefaultView()},
	}, nil
}

// UpdateTopic updates the fields on a topic.
func (r *MutationResolver) UpdateTopic(
	ctx context.Context, input models.UpdateTopicInput,
) (*models.UpdateTopicPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	c := services.New(r.DB, actor, nil)

	topic, err := models.Topics(
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.InnerJoin("organization_members om on r.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", input.ID, actor.ID),
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("No topic %s is visible to %s", input.ID, actor.Summary())
		return nil, err
	}
	log.Printf("%s attempting to update %s", actor.Summary(), topic.Summary())

	result, err := c.UpdateTopic(ctx, topic, input.Name, input.Description)
	if err != nil {
		log.Printf("There was a problem updating %s", topic.Summary())
		return nil, err
	}

	return &models.UpdateTopicPayload{
		Alerts: result.Alerts,
		Topic:  &models.TopicValue{topic, false, actor.DefaultView()},
	}, nil
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(
	ctx context.Context, input models.UpsertLinkInput,
) (*models.UpsertLinkPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	var result *services.UpsertLinkResult
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err := findRepo(ctx, tx, actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			log.Printf("Repo not found for link: %s/%s", input.OrganizationLogin, input.RepositoryName)
			return err
		}

		s := services.New(tx, actor, r.Fetcher)
		result, err = s.UpsertLink(
			ctx, repo, input.URL, input.Title, input.AddParentTopicIds,
		)

		return err
	})

	if err != nil {
		return nil, err
	}

	if result.Link == nil {
		return &models.UpsertLinkPayload{Alerts: result.Alerts}, nil
	}

	return &models.UpsertLinkPayload{
		Alerts: result.Alerts,
		LinkEdge: &models.LinkEdge{
			Node: &models.LinkValue{result.Link, result.LinkCreated, actor.DefaultView()},
		},
	}, nil
}

// UpdateLinkTopics sets the parent topics on a link.
func (r *MutationResolver) UpdateLinkTopics(
	ctx context.Context, input models.UpdateLinkTopicsInput,
) (*models.UpdateLinkTopicsPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	link, err := models.FindLink(ctx, r.DB, input.LinkID)
	if err != nil {
		return nil, err
	}

	topics := util.TopicsFromIds(input.ParentTopicIds)
	if err = link.SetParentTopics(ctx, r.DB, false, topics...); err != nil {
		return nil, err
	}

	if err = link.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.UpdateLinkTopicsPayload{
		Link: &models.LinkValue{link, false, actor.DefaultView()},
	}, nil
}

// UpdateTopicParentTopics sets the parent topics on a topic.
func (r *MutationResolver) UpdateTopicParentTopics(
	ctx context.Context, input models.UpdateTopicParentTopicsInput,
) (*models.UpdateTopicParentTopicsPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	var result *services.UpdateTopicParentTopicsResult
	var topic *models.Topic
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: actor}

		if topic, err = models.FindTopic(ctx, tx, input.TopicID); err != nil {
			return err
		}

		result, err = c.UpdateTopicParentTopics(ctx, topic, input.ParentTopicIds)
		return err
	})

	if err != nil {
		return nil, err
	}

	return &models.UpdateTopicParentTopicsPayload{
		Alerts: result.Alerts,
		Topic:  &models.TopicValue{result.Topic, false, actor.DefaultView()},
	}, nil
}
