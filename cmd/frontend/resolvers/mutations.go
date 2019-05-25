package resolvers

import (
	"context"
	"database/sql"
	"errors"
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

// AddSynonym adds a synonym to a topic.
func (r *MutationResolver) AddSynonym(
	ctx context.Context, input models.AddSynonymInput,
) (*models.AddSynonymPayload, error) {
	var result *services.AddSynonymResult
	var err error

	topic, err := models.Topics(
		qm.InnerJoin("organization_members om on topics.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", input.TopicID, r.Actor.ID),
	).One(ctx, r.DB)

	if err != nil {
		return nil, err
	}

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: r.Actor}

		result, err = c.AddSynonym(ctx, topic, input.Name, input.Locale)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed to create a synonym %s for topic %s: %s", r.Actor.Summary(), input.Name,
			input.TopicID, err,
		)
	}

	view, err := newViewFromTopic(ctx, r.DB, topic)
	if err != nil {
		return nil, err
	}

	return &models.AddSynonymPayload{
		Alerts:      result.Alerts,
		SynonymEdge: &models.SynonymEdge{Node: *result.Synonym},
		Topic:       &models.TopicValue{Topic: topic, View: view},
	}, nil
}

// DeleteLink sets the parent topics on a topic.
func (r *MutationResolver) DeleteLink(
	ctx context.Context, input models.DeleteLinkInput,
) (*models.DeleteLinkPayload, error) {
	if r.Actor.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	link, err := models.Links(
		qm.InnerJoin("organization_members om on links.organization_id = om.organization_id"),
		qm.Where("links.id = ? and om.user_id = ?", input.LinkID, r.Actor.ID),
		qm.Load("Repository"),
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("There was a problem looking up link: %s", input.LinkID)
		return nil, err
	}
	repo := link.R.Repository

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: r.Actor}

		_, err := c.DeleteLink(ctx, repo, link)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed to delete link %s from repo %s: %s", r.Actor.Summary(), link.Summary(),
			repo.Summary(), err,
		)
	}

	return &models.DeleteLinkPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedLinkID:    input.LinkID,
	}, nil
}

// DeleteSynonym deletes a topic synonym.
func (r *MutationResolver) DeleteSynonym(
	ctx context.Context, input models.DeleteSynonymInput,
) (*models.DeleteSynonymPayload, error) {
	if r.Actor.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	var result *services.DeleteSynonymResult

	synonym, err := models.Synonyms(
		qm.InnerJoin("topics t on synonyms.topic_id = t.id"),
		qm.InnerJoin("organization_members om on t.organization_id = om.organization_id"),
		qm.Where("synonyms.id = ? and om.user_id = ?", input.SynonymID, r.Actor.ID),
	).One(ctx, r.DB)

	if err != nil {
		log.Printf("There was a problem looking up synonym %s: %s", input.SynonymID, err)
		return nil, err
	}

	topic, err := synonym.Topic().One(ctx, r.DB)
	if err != nil {
		log.Printf("No topic found for synonym %s: %s", input.SynonymID, err)
		return nil, err
	}

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: r.Actor}
		result, err = c.DeleteSynonym(ctx, synonym)
		if err != nil {
			return err
		}
		return nil
	})

	if err != nil {
		log.Printf("There was a problem deleteting synonym %s: %s", input.SynonymID, err)
		return nil, err
	}

	view, err := newViewFromTopic(ctx, r.DB, topic)
	if err != nil {
		log.Printf("Could not create a view for topic %s: %s", topic.ID, err)
		return nil, err
	}

	if !result.Success {
		return &models.DeleteSynonymPayload{
			Alerts:           result.Alerts,
			ClientMutationID: input.ClientMutationID,
			DeletedSynonymID: nil,
			Topic:            &models.TopicValue{Topic: result.Topic, View: view},
		}, nil
	}

	return &models.DeleteSynonymPayload{
		Alerts:           result.Alerts,
		ClientMutationID: input.ClientMutationID,
		DeletedSynonymID: &input.SynonymID,
		Topic:            &models.TopicValue{Topic: result.Topic, View: view},
	}, nil
}

// DeleteTopic deletes a topic.
func (r *MutationResolver) DeleteTopic(
	ctx context.Context, input models.DeleteTopicInput,
) (*models.DeleteTopicPayload, error) {
	if r.Actor.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	topic, err := models.Topics(
		qm.InnerJoin("organization_members om on topics.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", input.TopicID, r.Actor.ID),
	).One(ctx, r.DB)
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

// SelectRepository selects the repository for the current user.
func (r *MutationResolver) SelectRepository(
	ctx context.Context, input models.SelectRepositoryInput,
) (*models.SelectRepositoryPayload, error) {
	repoID := input.RepositoryID
	log.Printf("Atempting to select repository %v for %#v", repoID, r.Actor)

	var err error
	var repo *models.Repository

	if repoID == nil {
		exists, err := r.Actor.SelectedRepository().Exists(ctx, r.DB)
		if exists {
			log.Printf("Unselecting repository from %s", r.Actor.ID)
			repo, err = r.Actor.SelectedRepository().One(ctx, r.DB)

			if err = r.Actor.RemoveSelectedRepository(ctx, r.DB, repo); err != nil {
				return nil, err
			}

			if err = r.Actor.Reload(ctx, r.DB); err != nil {
				return nil, err
			}
		}
		return &models.SelectRepositoryPayload{nil, *r.Actor}, nil
	}

	repo = &models.Repository{ID: *repoID}
	log.Printf("Selecting repository %s for user %s", repo.ID, r.Actor.ID)
	if err = r.Actor.SetSelectedRepository(ctx, r.DB, false, repo); err != nil {
		return nil, err
	}

	log.Printf("Reloading repo %s", repo.ID)
	if err = repo.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	log.Printf("Reloading user %s", r.Actor.ID)
	if err = r.Actor.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.SelectRepositoryPayload{repo, *r.Actor}, nil
}

// UpsertTopic creates a new topic.
func (r *MutationResolver) UpsertTopic(
	ctx context.Context, input models.UpsertTopicInput,
) (*models.UpsertTopicPayload, error) {
	var result *services.UpsertTopicResult
	var err error
	var repo *models.Repository

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err = findRepo(ctx, tx, r.Actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			return err
		}

		c := services.Connection{Exec: tx, Actor: r.Actor}
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
		return nil, err
	}

	if result.Topic == nil {
		return &models.UpsertTopicPayload{Alerts: result.Alerts}, nil
	}

	return &models.UpsertTopicPayload{
		Alerts: result.Alerts,
		TopicEdge: &models.TopicEdge{
			Node: models.TopicValue{result.Topic, result.TopicCreated, r.Actor.DefaultView()},
		},
	}, nil
}

// UpdateTopic updates the fields on a topic.
func (r *MutationResolver) UpdateTopic(
	ctx context.Context, input models.UpdateTopicInput,
) (*models.UpdateTopicPayload, error) {
	s := services.Connection{Exec: r.DB, Actor: r.Actor}

	topic, err := models.Topics(
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.InnerJoin("organization_members om on r.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", input.ID, r.Actor.ID),
	).One(ctx, r.DB)
	if err != nil {
		log.Printf("No topic %s is visible to %s", input.ID, r.Actor.Summary())
		return nil, err
	}
	log.Printf("%s attempting to update %s", r.Actor.Summary(), topic.Summary())

	result, err := s.UpdateTopic(ctx, topic, input.Name, input.Description)
	if err != nil {
		log.Printf("There was a problem updating %s", topic.Summary())
		return nil, err
	}

	return &models.UpdateTopicPayload{
		Alerts: result.Alerts,
		Topic:  models.TopicValue{topic, false, r.Actor.DefaultView()},
	}, nil
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(
	ctx context.Context, input models.UpsertLinkInput,
) (*models.UpsertLinkPayload, error) {
	var result *services.UpsertLinkResult
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err := findRepo(ctx, tx, r.Actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			log.Printf("Repo not found for link: %s/%s", input.OrganizationLogin, input.RepositoryName)
			return err
		}

		s := services.New(tx, r.Actor, r.Fetcher)
		result, err = s.UpsertLink(
			ctx,
			repo,
			input.URL,
			input.Title,
			input.AddParentTopicIds,
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
			Node: models.LinkValue{result.Link, result.LinkCreated, r.Actor.DefaultView()},
		},
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

	topics := util.TopicsFromIds(input.ParentTopicIds)
	if err = link.SetParentTopics(ctx, r.DB, false, topics...); err != nil {
		return nil, err
	}

	if err = link.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.UpdateLinkTopicsPayload{
		Link: models.LinkValue{link, false, r.Actor.DefaultView()},
	}, nil
}

// UpdateTopicParentTopics sets the parent topics on a topic.
func (r *MutationResolver) UpdateTopicParentTopics(
	ctx context.Context, input models.UpdateTopicParentTopicsInput,
) (*models.UpdateTopicParentTopicsPayload, error) {
	var result *services.UpdateTopicParentTopicsResult
	var topic *models.Topic
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: r.Actor}

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
		Topic:  models.TopicValue{result.Topic, false, r.Actor.DefaultView()},
	}, nil
}
