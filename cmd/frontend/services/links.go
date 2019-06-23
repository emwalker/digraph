package services

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
	"github.com/emwalker/digraph/cmd/frontend/text"
	"github.com/emwalker/digraph/cmd/frontend/util"
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

// UpsertLinkResult holds the result of an UpsertLink call.
type UpsertLinkResult struct {
	Alerts      []*models.Alert
	Cleanup     CleanupFunc
	Link        *models.Link
	LinkCreated bool
}

func (c Connection) providedOrFetchedTitle(url string, providedTitle *string) (string, error) {
	if providedTitle != nil && *providedTitle != "" {
		return *providedTitle, nil
	}

	log.Print("Fetching title of ", url)
	pageInfo, err := c.Fetcher.FetchPage(url)
	if err != nil {
		return "", err
	}

	if pageInfo.Title != nil {
		return *pageInfo.Title, nil
	}

	return "", nil
}

func (c Connection) addParentTopicsToLink(
	ctx context.Context, link *models.Link, parentTopicIds []string,
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
	).All(ctx, c.Exec)

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

	topics := util.TopicsFromIds(insertIds)
	return link.AddParentTopics(ctx, c.Exec, false, topics...)
}

func (c Connection) addTopics(
	ctx context.Context, repo *models.Repository, link *models.Link, parentTopicIds []string,
) error {
	existingTopicCount, err := link.ParentTopics().Count(ctx, c.Exec)
	if err != nil {
		log.Printf("Failed to query parent topic count for link: %#v", link)
		return err
	}

	if len(parentTopicIds) < 1 && existingTopicCount < 1 {
		var rootTopic *models.Topic
		if rootTopic, err = repo.Topics(qm.Where("root")).One(ctx, c.Exec); err != nil {
			log.Printf("Could not find root topic for repo %s", repo.ID)
			return err
		}
		parentTopicIds = append(parentTopicIds, rootTopic.ID)
	}

	if err = c.addParentTopicsToLink(ctx, link, parentTopicIds); err != nil {
		log.Printf("Failed to add parent topics to link %#v", link)
		return err
	}

	return nil
}

func (c Connection) addUserLinkReview(
	ctx context.Context, repo *models.Repository, link *models.Link,
) error {
	review := models.UserLinkReview{
		LinkID: link.ID,
		UserID: c.Actor.ID,
	}

	err := review.Upsert(
		ctx, c.Exec, true, []string{"link_id", "user_id"}, boil.Whitelist("reviewed_at"), boil.Infer(),
	)

	if err != nil {
		return err
	}

	return nil
}

func (c Connection) logUserLinkAction(
	ctx context.Context, repo *models.Repository, actor *models.User, link *models.Link, action string,
	newTopicIds []string,
) (*models.UserLink, error) {
	// Log the upsert
	userLink := models.UserLink{
		OrganizationID: repo.OrganizationID,
		RepositoryID:   repo.ID,
		LinkID:         link.ID,
		UserID:         actor.ID,
		Action:         action,
	}

	if err := userLink.Insert(ctx, c.Exec, boil.Infer()); err != nil {
		log.Printf("Failed to add a row to user_links: %s", err)
		return nil, err
	}

	for _, topicID := range newTopicIds {
		userLinkTopic := models.UserLinkTopic{
			UserLinkID: userLink.ID,
			Action:     models.TopicActionTopicAdded,
			TopicID:    topicID,
		}

		if err := userLinkTopic.Insert(ctx, c.Exec, boil.Infer()); err != nil {
			log.Printf("Failed to add a row to user_link_topics (%s, %s): %s", userLink.ID, topicID, err)
			return nil, err
		}
	}

	return &userLink, nil
}

func (c Connection) updateLink(ctx context.Context, link *models.Link) error {
	_, err := link.Update(ctx, c.Exec, boil.Whitelist("title"))
	if err != nil {
		log.Printf("There was a problem updating link %s: %s", link.Summary(), err)
	}
	return err
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
func (c Connection) UpsertLink(
	ctx context.Context, repo *models.Repository, providedURL string, providedTitle *string,
	parentTopicIds []string,
) (*UpsertLinkResult, error) {
	var alerts []*models.Alert
	existing := false

	url, err := pageinfo.NewURL(providedURL)
	if err != nil {
		log.Printf("Unable to normalize url: %s", err)
		alerts = append(
			alerts, models.NewAlert(models.AlertTypeWarn, err.Error()),
		)
		return &UpsertLinkResult{
			Alerts:  alerts,
			Cleanup: func() error { return nil },
		}, nil
	}

	link, err := repo.Links(qm.Where("sha1 like ?", url.Sha1)).One(ctx, c.Exec)
	if err != nil && err.Error() != "sql: no rows in result set" {
		log.Printf("Failed to query for existing link with sha1 %s: %s", url.Sha1, err)
		return nil, err
	}

	if link == nil {
		log.Printf("Link not found, fetching page: %s", url.CanonicalURL)

		title, err := c.providedOrFetchedTitle(url.CanonicalURL, providedTitle)
		if err != nil {
			log.Printf("Failed to fetch title for %s: %s", url.CanonicalURL, err)
			return nil, err
		}

		link = &models.Link{
			OrganizationID: repo.OrganizationID,
			RepositoryID:   repo.ID,
			Sha1:           url.Sha1,
			Title:          text.Squash(title),
			URL:            url.CanonicalURL,
		}

		err = link.Upsert(
			ctx, c.Exec, true, []string{"repository_id", "sha1"}, boil.Whitelist("url", "title"), boil.Infer(),
		)

		if err != nil {
			log.Printf("Failed to upsert link: %#v", link)
			return nil, err
		}
	} else {
		log.Printf("Link found: %s", url.CanonicalURL)
		existing = true

		if providedTitle == nil {
			alerts = []*models.Alert{
				models.NewAlert(models.AlertTypeSuccess, fmt.Sprintf("An existing link %s was found", providedURL)),
			}
		} else {
			if *providedTitle == "" {
				log.Printf("Provided title empty, updating link after re-fetching page: %s", link.Summary())
				title, err := c.providedOrFetchedTitle(url.CanonicalURL, providedTitle)
				if err != nil {
					return nil, err
				}

				link.Title = title
				if err = c.updateLink(ctx, link); err != nil {
					return nil, err
				}
			} else if *providedTitle != link.Title {
				log.Printf("Provided title and existing differ, updating to new title: %s", *providedTitle)

				link.Title = *providedTitle
				if err = c.updateLink(ctx, link); err != nil {
					return nil, err
				}
			} else {
				log.Printf("Provided title and existing title the same, doing nothing: %s", *providedTitle)
			}
		}
	}

	if err = c.addTopics(ctx, repo, link, parentTopicIds); err != nil {
		log.Printf("There was a problem adding topics to link %s: %s", link.Summary(), err)
		return nil, err
	}

	userLink, err := c.logUserLinkAction(ctx, repo, c.Actor, link, models.ActionUpsertLink, parentTopicIds)
	if err != nil {
		return nil, err
	}

	if err = c.addUserLinkReview(ctx, repo, link); err != nil {
		log.Printf("There was a problem creating a user link review record: %s", err)
		return nil, err
	}

	cleanup := func() error {
		if !existing {
			log.Printf("Deleteing link %s", link.ID)
			if _, err = link.Delete(ctx, c.Exec); err != nil {
				return err
			}
		}

		if _, err := userLink.Delete(ctx, c.Exec); err != nil {
			return err
		}

		return nil
	}

	if err = link.Reload(ctx, c.Exec); err != nil {
		log.Printf("Failed to reload link %s: %s", link.Summary(), err)
		return nil, err
	}

	return &UpsertLinkResult{
		Alerts:      alerts,
		Cleanup:     cleanup,
		Link:        link,
		LinkCreated: !existing,
	}, nil
}
