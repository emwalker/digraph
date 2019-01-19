package resolvers

import (
	"context"
	"database/sql"
	"errors"
	"log"
	"os"

	"github.com/emwalker/digraph/internal/models"
	"github.com/emwalker/digraph/internal/services"
	"github.com/emwalker/digraph/internal/util"
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
	user := getCurrentUser(ctx, r.DB)
	if user.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	link, err := models.Links(
		qm.InnerJoin("organization_members om on links.organization_id = om.organization_id"),
		qm.Where("links.id = ? and om.user_id = ?", input.LinkID, user.ID),
	).One(ctx, r.DB)
	if err != nil {
		log.Printf("There was a problem looking up link: %s", input.LinkID)
		return nil, err
	}

	if _, err = link.Delete(ctx, r.DB); err != nil {
		log.Printf("There was a problem deleting link: %#v", link)
		return nil, err
	}

	return &models.DeleteLinkPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedLinkID:    input.LinkID,
	}, nil
}

// DeleteTopic sets the parent topics on a topic.
func (r *MutationResolver) DeleteTopic(
	ctx context.Context, input models.DeleteTopicInput,
) (*models.DeleteTopicPayload, error) {
	user := getCurrentUser(ctx, r.DB)
	if user.IsGuest() {
		return nil, errNoAnonymousMutations
	}

	topic, err := models.Topics(
		qm.InnerJoin("organization_members om on topics.organization_id = om.organization_id"),
		qm.Where("topics.id = ? and om.user_id = ?", input.TopicID, user.ID),
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
	user := getCurrentUser(ctx, r.DB)
	log.Printf("Atempting to select repository %v for %#v", repoID, user)

	var err error
	var repo *models.Repository

	if repoID == nil {
		exists, err := user.SelectedRepository().Exists(ctx, r.DB)
		if exists {
			log.Printf("Unselecting repository from %s", user.ID)
			repo, err = user.SelectedRepository().One(ctx, r.DB)

			if err = user.RemoveSelectedRepository(ctx, r.DB, repo); err != nil {
				return nil, err
			}

			if err = user.Reload(ctx, r.DB); err != nil {
				return nil, err
			}
		}
		return &models.SelectRepositoryPayload{nil, user}, nil
	}

	repo = &models.Repository{ID: *repoID}
	log.Printf("Selecting repository %s for user %s", repo.ID, user.ID)
	if err = user.SetSelectedRepository(ctx, r.DB, false, repo); err != nil {
		return nil, err
	}

	log.Printf("Reloading repo %s", repo.ID)
	if err = repo.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	log.Printf("Reloading user %s", user.ID)
	if err = user.Reload(ctx, r.DB); err != nil {
		return nil, err
	}

	return &models.SelectRepositoryPayload{repo, user}, nil
}

// UpsertTopic creates a new topic.
func (r *MutationResolver) UpsertTopic(
	ctx context.Context, input models.UpsertTopicInput,
) (*models.UpsertTopicPayload, error) {
	var result *services.UpsertTopicResult
	var err error
	var repo *models.Repository
	actor := getCurrentUser(ctx, r.DB)

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err = findRepo(ctx, tx, &actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			return err
		}

		c := services.Connection{Exec: tx, Actor: &actor}
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
			Node: models.TopicValue{result.Topic, result.TopicCreated},
		},
	}, nil
}

// UpdateTopic updates the fields on a topic.
func (r *MutationResolver) UpdateTopic(
	ctx context.Context, input models.UpdateTopicInput,
) (*models.UpdateTopicPayload, error) {
	actor := getCurrentUser(ctx, r.DB)
	s := services.Connection{Exec: r.DB, Actor: &actor}

	topic, err := models.Topics(
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.Where("topics.id = ? and r.owner_id = ?", input.ID, actor.ID),
	).One(ctx, r.DB)
	if err != nil {
		return nil, err
	}

	result, err := s.UpdateTopic(ctx, topic, input.Name, input.Description)
	if err != nil {
		return nil, err
	}

	return &models.UpdateTopicPayload{
		Alerts: result.Alerts,
		Topic:  models.TopicValue{topic, false},
	}, nil
}

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(
	ctx context.Context, input models.UpsertLinkInput,
) (*models.UpsertLinkPayload, error) {
	actor := getCurrentUser(ctx, r.DB)
	var result *services.UpsertLinkResult
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		repo, err := findRepo(ctx, tx, &actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			log.Printf("Repo not found for link: %s/%s", input.OrganizationLogin, input.RepositoryName)
			return err
		}

		s := services.New(tx, &actor)
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
		Alerts:   result.Alerts,
		LinkEdge: &models.LinkEdge{Node: models.LinkValue{result.Link, result.LinkCreated}},
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
		Link: models.LinkValue{link, false},
	}, nil
}

// UpdateTopicParentTopics sets the parent topics on a topic.
func (r *MutationResolver) UpdateTopicParentTopics(
	ctx context.Context, input models.UpdateTopicParentTopicsInput,
) (*models.UpdateTopicParentTopicsPayload, error) {
	actor := getCurrentUser(ctx, r.DB)
	var result *services.UpdateTopicParentTopicsResult
	var topic *models.Topic
	var err error

	err = transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: &actor}

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
		Topic:  models.TopicValue{result.Topic, false},
	}, nil
}
