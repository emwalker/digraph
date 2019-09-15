package resolvers

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/emwalker/digraph/cmd/frontend/util"
	perrors "github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// MutationResolver holds configuration information for a mutation.
type MutationResolver struct {
	*Resolver
}

// Special errors.
var (
	ErrNoAnonymousMutations = errors.New("anonymous users cannot make updates or deletions")
	ErrUnauthorized         = errors.New("you are not allowed to do that")
)

func init() {
	log.SetOutput(os.Stdout)
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

type createSessionFunc func(*sql.Tx, *models.User) (*services.CreateSessionResult, error)

func (r *MutationResolver) sessionPayload(
	ctx context.Context, serverSecret string, createSession createSessionFunc,
) (*models.CreateSessionPayload, error) {
	rc := GetRequestContext(ctx)
	actor := rc.Viewer()

	if !rc.InitiatedByServer(serverSecret) {
		log.Printf("Session creation initiated by %s rather than the server", actor)
		return nil, ErrUnauthorized
	}
	log.Printf("Request comes from server, creating session for %s", actor)

	var result *services.CreateSessionResult

	err := queries.Transact(r.DB, func(tx *sql.Tx) error {
		var err error
		result, err = createSession(tx, actor)
		return err
	})

	if err != nil {
		log.Printf("There was a problem creating a session: %s", err)
		return nil, perrors.Wrap(err, "resolvers: failed to create session")
	}

	return &models.CreateSessionPayload{
		Alerts:      result.Alerts,
		UserEdge:    &models.UserEdge{Node: result.User},
		SessionEdge: &models.SessionEdge{Node: result.Session},
	}, nil
}

// CreateGithubSession creates a new session for the user passed in, possibly alongside any existing
// sessions.  If the user is not yet in the database, a new user is created.  Sessions are destroyed
// using DestroySession, which is called when someone logs out of the client.
func (r *MutationResolver) CreateGithubSession(
	ctx context.Context, input models.CreateGithubSessionInput,
) (*models.CreateSessionPayload, error) {
	return r.sessionPayload(ctx, input.ServerSecret, func(tx *sql.Tx, actor *models.User) (*services.CreateSessionResult, error) {
		c := services.New(tx, actor, nil)
		return c.CreateGithubSession(
			ctx, input.Name, input.PrimaryEmail, input.GithubUsername, input.GithubAvatarURL,
		)
	})
}

// CreateGoogleSession creates a new session for the user passed in.  See CreateGithubSession for
// details.
func (r *MutationResolver) CreateGoogleSession(
	ctx context.Context, input models.CreateGoogleSessionInput,
) (*models.CreateSessionPayload, error) {
	return r.sessionPayload(ctx, input.ServerSecret, func(tx *sql.Tx, actor *models.User) (*services.CreateSessionResult, error) {
		c := services.New(tx, actor, nil)
		return c.CreateGoogleSession(
			ctx, input.Name, input.PrimaryEmail, input.GoogleProfileID, input.GoogleAvatarURL,
		)
	})
}

// DeleteAccount deletes the user account and any private data associated with it.  Links and topics
// added to the public repo will not be removed, but the association between the user and the links
// and topics will no longer exist.
func (r *MutationResolver) DeleteAccount(
	ctx context.Context, input models.DeleteAccountInput,
) (*models.DeleteAccountPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, ErrNoAnonymousMutations
	}

	if actor.ID != input.UserID {
		return nil, fmt.Errorf("not allowed to delete account %s", input.UserID)
	}

	c := services.Connection{Exec: r.DB, Actor: actor}
	result, err := c.DeleteAccount(ctx, actor)

	if err != nil {
		return nil, err
	}

	return &models.DeleteAccountPayload{
		Alerts:        result.Alerts,
		DeletedUserID: actor.ID,
	}, nil
}

// DeleteLink sets the parent topics on a topic.
func (r *MutationResolver) DeleteLink(
	ctx context.Context, input models.DeleteLinkInput,
) (*models.DeleteLinkPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, ErrNoAnonymousMutations
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

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: actor}

		_, err := c.DeleteLink(ctx, repo, link)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed to delete link %s from repo %s: %s", actor, link, repo, err,
		)
	}

	return &models.DeleteLinkPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedLinkID:    input.LinkID,
	}, nil
}

// DeleteSession deletes a user session.
func (r *MutationResolver) DeleteSession(
	ctx context.Context, input models.DeleteSessionInput,
) (*models.DeleteSessionPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	session, err := actor.Sessions(qm.Where("id = ?", input.SessionID)).One(ctx, r.DB)
	if err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to fetch session")
	}

	if _, err = session.Delete(ctx, r.DB); err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to delete session")
	}

	return &models.DeleteSessionPayload{DeletedSessionID: input.SessionID}, nil
}

// DeleteTopic deletes a topic.
func (r *MutationResolver) DeleteTopic(
	ctx context.Context, input models.DeleteTopicInput,
) (*models.DeleteTopicPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, ErrNoAnonymousMutations
	}

	topic, err := fetchTopic(ctx, r.DB, input.TopicID, actor)
	if err != nil {
		log.Printf("There was a problem looking up topic: %s", input.TopicID)
		return nil, perrors.Wrap(err, "resolvers: failed to fetch topic")
	}

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
		c := services.New(tx, actor, nil)

		if _, err = c.DeleteTopic(ctx, topic); err != nil {
			log.Printf("There was a problem deleting topic: %#v", topic)
			return err
		}

		return nil
	})

	if err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to delete topic")
	}

	return &models.DeleteTopicPayload{
		ClientMutationID: input.ClientMutationID,
		DeletedTopicID:   input.TopicID,
	}, nil
}

// DeleteTopicTimeRange deletes a topic.
func (r *MutationResolver) DeleteTopicTimeRange(
	ctx context.Context, input models.DeleteTopicTimeRangeInput,
) (*models.DeleteTopicTimeRangePayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	if actor.IsGuest() {
		return nil, ErrNoAnonymousMutations
	}

	topic, err := fetchTopic(ctx, r.DB, input.TopicID, actor)
	if err != nil {
		log.Printf("There was a problem looking up topic: %s", input.TopicID)
		return nil, err
	}

	c := services.Connection{Exec: r.DB, Actor: actor}
	result, err := c.DeleteTopicTimeRange(ctx, topic)
	if err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to delete time range")
	}

	return &models.DeleteTopicTimeRangePayload{
		DeletedTimeRangeID: result.DeletedTimeRangeID,
		Topic:              &models.TopicValue{Topic: result.Topic, View: actor.DefaultView()},
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
		log.Printf("Did not find link %s in the repos visible to %s: %s", input.LinkID, actor, err)
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

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
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

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
		c := services.Connection{Exec: tx, Actor: actor}

		result, err = c.UpdateSynonyms(ctx, topic, synonyms)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		log.Printf(
			"%s failed update synonyms (%v) topic %s: %s", actor, synonyms, topic.ID, err,
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

	topic, err := models.Topics(queries.Topic(actor.ID, input.ID)...).One(ctx, r.DB)
	if err != nil {
		log.Printf("No topic %s is visible to %s", input.ID, actor)
		return nil, err
	}

	log.Printf("%s attempting to update %s", actor, topic)
	result, err := c.UpdateTopic(ctx, topic, input.Name, input.Description)
	if err != nil {
		log.Printf("There was a problem updating %s", topic)
		return nil, err
	}

	return &models.UpdateTopicPayload{
		Alerts: result.Alerts,
		Topic:  &models.TopicValue{topic, false, actor.DefaultView()},
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

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
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

// UpsertLink adds a new link to the database.
func (r *MutationResolver) UpsertLink(
	ctx context.Context, input models.UpsertLinkInput,
) (*models.UpsertLinkPayload, error) {
	actor := GetRequestContext(ctx).Viewer()

	var result *services.UpsertLinkResult
	var err error

	err = queries.Transact(r.DB, func(tx *sql.Tx) error {
		repo, err := findRepo(ctx, tx, actor, input.OrganizationLogin, input.RepositoryName)
		if err != nil {
			log.Printf("resolvers.UpsertLink: repo %s/%s not found", input.OrganizationLogin, input.RepositoryName)
			return perrors.Wrap(err, "resolvers.UpsertLink")
		}

		s := services.New(tx, actor, r.Fetcher)
		result, err = s.UpsertLink(
			ctx, repo, input.URL, input.Title, input.AddParentTopicIds,
		)

		return perrors.Wrap(err, "resolvers.UpsertLink")
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

// UpsertTopicTimeRange adds a timeline to a topic.
func (r *MutationResolver) UpsertTopicTimeRange(
	ctx context.Context, input models.UpsertTopicTimeRangeInput,
) (*models.UpsertTopicTimeRangePayload, error) {
	actor := GetRequestContext(ctx).Viewer()
	c := services.Connection{Exec: r.DB, Actor: actor}

	topic, err := models.Topics(queries.Topic(actor.ID, input.TopicID)...).One(ctx, r.DB)
	if err != nil {
		return nil, perrors.Wrap(err, "resolveres: topic not found")
	}

	startsAt, err := time.Parse(time.RFC3339, input.StartsAt)
	if err != nil {
		return nil, perrors.Wrap(err, "resolveres: failed to parse startsAt")
	}

	result, err := c.UpsertTopicTimeRange(ctx, topic, startsAt, nil, input.PrefixFormat)
	if err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to upsert topic timline")
	}

	timeline := &models.TimeRange{
		StartsAt:     startsAt.Format(time.RFC3339),
		PrefixFormat: models.TimeRangePrefixFormat(result.TimeRange.PrefixFormat),
	}

	if err = topic.Reload(ctx, r.DB); err != nil {
		return nil, perrors.Wrap(err, "resolvers: failed to reload topic")
	}

	return &models.UpsertTopicTimeRangePayload{
		Topic:         &models.TopicValue{Topic: topic, View: actor.DefaultView()},
		TimeRangeEdge: &models.TimeRangeEdge{Node: timeline},
	}, nil
}
