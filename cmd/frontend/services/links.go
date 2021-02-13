package services

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/queries"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/emwalker/digraph/cmd/frontend/text"
	"github.com/emwalker/digraph/cmd/frontend/util"
	"github.com/pkg/errors"
	"github.com/volatiletech/null"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// DeleteLinkResult holds the result of a DeleteLink call.
type DeleteLinkResult struct {
	Cleanup       CleanupFunc
	DeletedLinkID string
}

// ReviewLinkResult holds the result of a ReviewLink call.
type ReviewLinkResult struct {
	Link   *models.Link
	Review *models.UserLinkReview
}

// DeleteLink removes a link from a repo.
func (c Connection) DeleteLink(
	ctx context.Context, repo *models.Repository, link *models.Link,
) (*DeleteLinkResult, error) {
	var err error

	if _, err = link.Delete(ctx, c.Exec); err != nil {
		log.Printf("There was a problem deleting link: %#v", link)
		return nil, err
	}

	cleanup := func() error {
		return nil
	}

	return &DeleteLinkResult{
		Cleanup:       cleanup,
		DeletedLinkID: link.ID,
	}, nil
}

// ReviewLink toggles whether a link has been reviewed.
func (c Connection) ReviewLink(
	ctx context.Context, link *models.Link, reviewed bool,
) (*ReviewLinkResult, error) {
	reviewedAt := time.Now()

	review := models.UserLinkReview{
		LinkID:     link.ID,
		UserID:     c.Actor.ID,
		ReviewedAt: null.TimeFrom(reviewedAt),
	}
	log.Printf("Marking link %s as reviewed at %v", link.ID, reviewedAt)

	err := review.Upsert(
		ctx, c.Exec, true, []string{"user_id", "link_id"}, boil.Whitelist("reviewed_at"), boil.Infer(),
	)

	if err != nil {
		log.Printf("Unable to upsert review: %s", err)
		return nil, err
	}

	return &ReviewLinkResult{Link: link, Review: &review}, nil
}

// UpsertLink adds a link if it does not yet exist in the database or updates information about
// the link if it is found.
type UpsertLink struct {
	Actor          *models.User
	Fetcher        pageinfo.Fetcher
	ParentTopicIds []string
	ProvidedTitle  *string
	ProvidedURL    string
	Repository     *models.Repository
}

// UpsertLinkResult holds the result of an UpsertLink call.
type UpsertLinkResult struct {
	Alerts      []*models.Alert
	Link        *models.Link
	LinkCreated bool
}

func (m UpsertLink) addUserLinkReview(
	ctx context.Context, exec boil.ContextExecutor, link *models.Link,
) error {
	review := models.UserLinkReview{
		LinkID: link.ID,
		UserID: m.Actor.ID,
	}

	err := review.Upsert(
		ctx, exec, true, []string{"link_id", "user_id"}, boil.Whitelist("reviewed_at"), boil.Infer(),
	)
	if err != nil {
		return err
	}

	return nil
}

func (m UpsertLink) logUserLinkAction(
	ctx context.Context, exec boil.ContextExecutor, link *models.Link, action string,
) (*models.UserLink, error) {
	// Log the upsert
	userLink := models.UserLink{
		OrganizationID: m.Repository.OrganizationID,
		RepositoryID:   m.Repository.ID,
		LinkID:         link.ID,
		UserID:         m.Actor.ID,
		Action:         action,
	}

	if err := userLink.Insert(ctx, exec, boil.Infer()); err != nil {
		log.Printf("Failed to add a row to user_links: %s", err)
		return nil, err
	}

	for _, topicID := range m.ParentTopicIds {
		userLinkTopic := models.UserLinkTopic{
			UserLinkID: userLink.ID,
			Action:     models.TopicActionTopicAdded,
			TopicID:    topicID,
		}

		if err := userLinkTopic.Insert(ctx, exec, boil.Infer()); err != nil {
			log.Printf("Failed to add a row to user_link_topics (%s, %s): %s", userLink.ID, topicID, err)
			return nil, err
		}
	}

	return &userLink, nil
}

func (m UpsertLink) updateLink(ctx context.Context, exec boil.ContextExecutor, link *models.Link) error {
	_, err := link.Update(ctx, exec, boil.Whitelist("title"))
	if err != nil {
		log.Printf("There was a problem updating link %s: %s", link, err)
	}
	return err
}

func (m UpsertLink) addTopics(
	ctx context.Context, exec boil.ContextExecutor, link *models.Link,
) error {
	parentTopicIds := m.ParentTopicIds

	existingTopicCount, err := link.ParentTopics().Count(ctx, exec)
	if err != nil {
		log.Printf("Failed to query parent topic count for link: %#v", link)
		return err
	}

	if len(parentTopicIds) < 1 && existingTopicCount < 1 {
		var rootTopic *models.Topic
		if rootTopic, err = m.Repository.Topics(qm.Where("root")).One(ctx, exec); err != nil {
			log.Printf("Could not find root topic for repo %s", m.Repository.ID)
			return err
		}
		parentTopicIds = append(parentTopicIds, rootTopic.ID)
	}

	log.Printf("Adding topic ids %s", parentTopicIds)
	for _, topicID := range parentTopicIds {
		sql := "select add_topic_to_link($1, $2)"
		log.Printf("adding topic to link: %s, %v", topicID, link)
		values := []interface{}{topicID, link.ID}
		_, err := exec.ExecContext(ctx, sql, values...)
		if err != nil {
			return errors.Wrap(err, "failed to upsert link")
		}
	}
	return nil
}

func (m UpsertLink) providedOrFetchedTitle(url string) (string, error) {
	providedTitle := m.ProvidedTitle
	if util.Present(providedTitle) {
		return *providedTitle, nil
	}

	log.Print("Fetching title of ", url)
	pageInfo, err := m.Fetcher.FetchPage(url)
	if err != nil {
		return "", err
	}

	if pageInfo.Title != nil {
		return *pageInfo.Title, nil
	}

	return "", nil
}

func (m UpsertLink) createLink(
	ctx context.Context, exec boil.ContextExecutor, url *pageinfo.URL,
) (*models.Link, error) {
	log.Printf("Link not found, fetching page: %s", url.CanonicalURL)

	title, err := m.providedOrFetchedTitle(url.CanonicalURL)
	if err != nil {
		log.Printf("Failed to fetch title for %s: %s", url.CanonicalURL, err)
		title = "Error fetching title"
	}

	link := &models.Link{
		OrganizationID: m.Repository.OrganizationID,
		RepositoryID:   m.Repository.ID,
		Sha1:           url.Sha1,
		Title:          text.Squash(title),
		URL:            url.CanonicalURL,
	}

	err = link.Upsert(
		ctx, exec, true, []string{"repository_id", "sha1"}, boil.Whitelist("url", "title"), boil.Infer(),
	)
	return link, err
}

// Call performs the mutation
func (m UpsertLink) Call(ctx context.Context, exec boil.ContextExecutor) (*UpsertLinkResult, error) {
	var alerts []*models.Alert
	existing := false

	url, err := pageinfo.NewURL(m.ProvidedURL)
	if err != nil {
		log.Printf("Unable to normalize url: %s", err)
		alerts = append(
			alerts, models.NewAlert(models.AlertTypeWarn, err.Error()),
		)
		return &UpsertLinkResult{
			Alerts: alerts,
		}, nil
	}

	link, err := m.Repository.Links(qm.Where("sha1 like ?", url.Sha1)).One(ctx, exec)
	if queries.IsRealError(err) {
		log.Printf("Failed to query for existing link with sha1 %s: %s", url.Sha1, err)
		return nil, errors.Wrap(err, "services.UpsertLink")
	}

	if link == nil {
		link, err = m.createLink(ctx, exec, url)
		if err != nil {
			log.Printf("Failed to upsert link: %#v", link)
			return nil, errors.Wrap(err, "services.UpsertLink")
		}
	} else {
		log.Printf("Link found: %s", url.CanonicalURL)
		providedTitle := m.ProvidedTitle
		existing = true

		if providedTitle == nil {
			alerts = []*models.Alert{
				models.NewAlert(models.AlertTypeSuccess, fmt.Sprintf("An existing link %s was found", m.ProvidedURL)),
			}
		} else {
			if *providedTitle == "" {
				log.Printf("Provided title empty, updating link after re-fetching page: %s", link)
				title, err := m.providedOrFetchedTitle(url.CanonicalURL)
				if err != nil {
					return nil, errors.Wrap(err, "services.UpsertLink")
				}

				link.Title = title
				if err = m.updateLink(ctx, exec, link); err != nil {
					return nil, errors.Wrap(err, "services.UpsertLink")
				}
			} else if *providedTitle != link.Title {
				log.Printf("Provided title and existing differ, updating to new title: %s", *providedTitle)

				link.Title = *providedTitle
				if err = m.updateLink(ctx, exec, link); err != nil {
					return nil, errors.Wrap(err, "services.UpsertLink")
				}
			} else {
				log.Printf("Provided title and existing title the same, not fetching title: %s", *providedTitle)
			}
		}
	}

	if err = m.addTopics(ctx, exec, link); err != nil {
		log.Printf("There was a problem adding topics %v to link %s: %s", m.ParentTopicIds, link, err)
		return nil, errors.Wrap(err, "services.UpsertLink")
	}

	_, err = m.logUserLinkAction(ctx, exec, link, models.ActionUpsertLink)
	if err != nil {
		return nil, errors.Wrap(err, "services.UpsertLink")
	}

	if err = m.addUserLinkReview(ctx, exec, link); err != nil {
		log.Printf("There was a problem creating a user link review record: %s", err)
		return nil, errors.Wrap(err, "services.UpsertLink")
	}

	if err = link.Reload(ctx, exec); err != nil {
		log.Printf("Failed to reload link %s: %s", link, err)
		return nil, errors.Wrap(err, "services.UpsertLink")
	}

	return &UpsertLinkResult{
		Alerts:      alerts,
		Link:        link,
		LinkCreated: !existing,
	}, nil
}

// UpdateLinkTopics is used to update the topics on a link.  If there are no topics, the link is added
// to the "Everything" topic.  The link_transitive_closure table is also updated.
type UpdateLinkTopics struct {
	Actor          *models.User
	ParentTopicIds []string
	Link           *models.LinkValue
}

// UpdateLinkTopicsResult holds the result of an updating of the topics on a link
type UpdateLinkTopicsResult struct {
	Link *models.LinkValue
}

// Call executes the mutation
func (s UpdateLinkTopics) Call(ctx context.Context, exec boil.ContextExecutor) (*UpdateLinkTopicsResult, error) {
	link := s.Link
	topicIds := s.ParentTopicIds

	if len(topicIds) < 1 {
		topicIds = append(topicIds, PublicRootTopicID)
	}

	values := []interface{}{link.ID}
	sql := "delete from link_transitive_closure where child_id = $1"
	if _, err := exec.ExecContext(ctx, sql, values...); err != nil {
		return nil, err
	}

	values = []interface{}{link.ID}
	sql = "delete from link_topics where child_id = $1"
	if _, err := exec.ExecContext(ctx, sql, values...); err != nil {
		return nil, err
	}

	for _, topicID := range topicIds {
		values := []interface{}{topicID, link.ID}
		sql := "select add_topic_to_link($1, $2)"
		if _, err := exec.ExecContext(ctx, sql, values...); err != nil {
			return nil, err
		}
	}

	if err := link.Reload(ctx, exec); err != nil {
		return nil, err
	}
	return &UpdateLinkTopicsResult{link}, nil
}
