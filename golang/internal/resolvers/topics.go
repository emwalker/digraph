package resolvers

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/emwalker/digraph/golang/internal/loaders"
	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/queries"
	"github.com/emwalker/digraph/golang/internal/queries/parser"
	"github.com/emwalker/digraph/golang/internal/resolvers/activity"
	"github.com/emwalker/digraph/golang/internal/services"
	"github.com/emwalker/digraph/golang/internal/util"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
	"github.com/volatiletech/sqlboiler/v4/types"
)

type topicResolver struct{ *Resolver }

// ByName provides a way to sort a topic by name.
type ByName struct {
	topics []*models.Topic
}

func (b ByName) Len() int {
	return len(b.topics)
}

func (b ByName) Swap(i, j int) {
	b.topics[i], b.topics[j] = b.topics[j], b.topics[i]
}

func (b ByName) Less(i, j int) bool {
	return b.topics[i].Name < b.topics[j].Name
}

func topicConnection(view *models.View, rows []*models.Topic, err error) (*models.TopicConnection, error) {
	if err != nil {
		return nil, err
	}

	edges := make([]*models.TopicEdge, len(rows))
	for i, topic := range rows {
		edges[i] = &models.TopicEdge{
			Node: &models.TopicValue{
				Topic:      topic,
				NewlyAdded: false,
				View:       view,
			},
		}
	}

	return &models.TopicConnection{
		Edges:    edges,
		PageInfo: &models.PageInfo{},
	}, nil
}

func availableTopics(
	ctx context.Context, exec boil.ContextExecutor, view *models.View, searchString *string, first *int,
	excludeTopicIds []string,
) (*models.TopicConnection, error) {
	mods := []qm.QueryMod{
		qm.InnerJoin("organizations o on o.id = topics.organization_id"),
		qm.InnerJoin("organization_members om on om.organization_id = o.id"),
		qm.InnerJoin("users u on om.user_id = u.id"),
		qm.Where("u.id = ?", view.ViewerID),
		qm.OrderBy("topics.name"),
		qm.Limit(limitFrom(first)),
	}

	if util.Present(searchString) {
		s := parser.Parse(searchString)
		whereClause := fmt.Sprintf(
			"to_tsvector('synonymsdict', topics.synonyms) @@ to_tsquery(%s)", s.EscapedPostgresTsQueryInput(),
		)
		mods = append(mods, qm.Where(whereClause))
	}

	if len(excludeTopicIds) > 0 {
		mods = append(mods, qm.Where("topics.id != all(?)", types.Array(excludeTopicIds)))
	}

	topics, err := models.Topics(mods...).All(ctx, exec)
	if err != nil {
		return nil, err
	}

	return topicConnection(view, topics, err)
}

// Activity returns a feed of actions that have recently been taken.
func (r *topicResolver) Activity(
	ctx context.Context, topic *models.TopicValue, first *int, after *string, last *int, before *string,
) (*models.ActivityLineItemConnection, error) {
	mods := topic.View.Filter([]qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Load("UserLinkTopics"),
		qm.Load("UserLinkTopics.Topic"),
		qm.Load("Link"),
		qm.Load("User"),
		qm.InnerJoin("repositories r on user_links.repository_id = r.id"),
		qm.InnerJoin("user_link_topics ult on user_links.id = ult.user_link_id"),
		qm.InnerJoin("link_transitive_closure ltc on user_links.link_id = ltc.child_id"),
		qm.WhereIn("ltc.parent_id = ?", topic.ID),
		qm.Limit(pageSizeOrDefault(first)),
	})

	userLinks, err := models.UserLinks(mods...).All(ctx, r.DB)
	if err != nil {
		return nil, errors.Wrap(err, "resolvers: failed to fetch user links")
	}

	logData := make([]activity.UpsertLink, len(userLinks))

	for i, userLink := range userLinks {
		topics := make([]activity.Topic, len(userLink.R.UserLinkTopics))

		for j, linkTopic := range userLink.R.UserLinkTopics {
			topic := linkTopic.R.Topic
			topics[j] = activity.Topic{Name: topic.Name, ID: topic.ID}
		}

		logData[i] = activity.UpsertLink{
			CreatedAt: userLink.CreatedAt,
			User:      activity.User{Name: userLink.R.User.DisplayName()},
			Link:      activity.Link{Title: userLink.R.Link.Title, URL: userLink.R.Link.URL},
			Topics:    topics,
		}
	}

	edges, err := activity.MakeEdges(logData)
	if err != nil {
		return nil, errors.Wrap(err, "resolvers.Activity")
	}

	return &models.ActivityLineItemConnection{Edges: edges}, nil
}

func (r *topicResolver) AvailableParentTopics(
	ctx context.Context, topic *models.TopicValue, searchString *string, first *int, after *string,
	last *int, before *string,
) (*models.TopicConnection, error) {
	return availableTopics(ctx, r.DB, topic.View, searchString, first, []string{topic.ID})
}

func (r *topicResolver) ChildTopics(
	ctx context.Context, topic *models.TopicValue, searchString *string, first *int, after *string,
	last *int, before *string,
) (*models.TopicConnection, error) {
	log.Printf("Fetching child topics for topic %s", topic.ID)

	mods := topic.View.Filter([]qm.QueryMod{
		qm.Load("ParentTopics"),
		qm.Load("ParentTopics.Timerange"),
		qm.Load("Repository"),
		qm.Load("Repository.Owner"),
		qm.Load("Timerange"),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
		qm.OrderBy("topics.name"),
	})

	if util.Present(searchString) {
		s := parser.Parse(searchString)
		whereClause := fmt.Sprintf(
			"to_tsvector('synonymsdict', topics.synonyms) @@ to_tsquery(%s)", s.EscapedPostgresTsQueryInput(),
		)
		mods = append(mods, qm.Where(whereClause))
	}

	topics, err := topic.ChildTopics(mods...).All(ctx, r.DB)
	return topicConnection(topic.View, topics, err)
}

// CreatedAt returns the time of the topic's creation.
func (r *topicResolver) CreatedAt(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.CreatedAt.Format(time.RFC3339), nil
}

// Description returns a description of the topic.
func (r *topicResolver) Description(_ context.Context, topic *models.TopicValue) (*string, error) {
	return topic.Description.Ptr(), nil
}

// DisplayName returns the name of the topic.  The name is obtained by finding the first synonym
// in the current locale.  If there is no synonym in the current locale, the first English synonym
// is returned.  If there is no English synonym, the first synonym is returned.
func (r *topicResolver) DisplayName(
	ctx context.Context, topic *models.TopicValue, includeTimeRange *bool,
) (string, error) {
	synonyms, err := topic.SynonymList()
	if err != nil {
		return "<name missing>", errors.Wrap(err, "resolvers: failed to fetch synonym list")
	}

	if includeTimeRange != nil && *includeTimeRange {
		timerange, err := queries.TimeRange(ctx, r.DB, topic.Topic)
		if err != nil {
			return "<name missing>", errors.Wrap(err, "resolvers: failed to fetch time range")
		}
		return services.DisplayName(timerange, synonyms, models.LocaleIdentifierEn)
	}

	return services.DisplayName(nil, synonyms, models.LocaleIdentifierEn)
}

func (r *topicResolver) ID(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.ID, nil
}

// Links returns a set of links.
func (r *topicResolver) Links(
	ctx context.Context, topic *models.TopicValue, first *int, after *string, last *int, before *string,
	searchString *string, reviewed, descendants *bool,
) (*models.LinkConnection, error) {
	log.Printf("resovlers: fetching links for topic %s", topic)
	viewer := GetRequestContext(ctx).Viewer()

	query := queries.LinkQuery{
		First:              first,
		IncludeDescendants: descendants != nil && *descendants,
		Reviewed:           reviewed,
		SearchString:       searchString,
		Topic:              topic.Topic,
		View:               topic.View,
		Viewer:             viewer,
	}
	rows, err := query.Fetch(ctx, r.DB)
	return linkConnection(topic.View, rows, len(rows), err)
}

// Loading is true if the topic is being loaded.  Only used on the client.
func (r *topicResolver) Loading(_ context.Context, topic *models.TopicValue) (bool, error) {
	return false, nil
}

func (r *topicResolver) Name(ctx context.Context, topic *models.TopicValue) (string, error) {
	return topic.Name, nil
}

// NewlyAdded returns true if the topic was just added.
func (r *topicResolver) NewlyAdded(_ context.Context, topic *models.TopicValue) (bool, error) {
	return topic.NewlyAdded, nil
}

// Organization returns an organization.
func (r *topicResolver) Organization(
	ctx context.Context, topic *models.TopicValue,
) (*models.Organization, error) {
	return loaders.GetOrg(ctx, topic.OrganizationID)
}

// ParentTopics returns a set of topics.
func (r *topicResolver) ParentTopics(
	ctx context.Context, topic *models.TopicValue, first *int, after *string, last *int, before *string,
) (*models.TopicConnection, error) {
	if topic.R != nil && len(topic.R.ParentTopics) > 0 {
		return topicConnection(topic.View, topic.R.ParentTopics, nil)
	}

	query := queries.TopicParentTopics{View: topic.View, Topic: topic.Topic}
	topics, err := query.Fetch(ctx, r.DB)
	return topicConnection(topic.View, topics, err)
}

// Repository returns the repostory of the topic.
func (r *topicResolver) Repository(
	ctx context.Context, topic *models.TopicValue,
) (*models.Repository, error) {
	return loaders.GetRepo(ctx, topic.RepositoryID)
}

// ResourcePath returns a path to the item.
func (r *topicResolver) ResourcePath(ctx context.Context, topic *models.TopicValue) (string, error) {
	repo, err := loaders.GetRepo(ctx, topic.RepositoryID)
	if err != nil {
		return "", err
	}

	org, err := loaders.GetOrg(ctx, topic.OrganizationID)
	if err != nil {
		return "", err
	}

	if repo.System {
		return fmt.Sprintf("/%s/topics/%s", org.Login, topic.ID), nil
	}
	return fmt.Sprintf("/%s/%s/topics/%s", org.Login, repo.Name, topic.ID), nil
}

func (r *topicResolver) Search(
	ctx context.Context, topic *models.TopicValue, searchString string, first *int, after *string,
	last *int, before *string,
) (*models.SearchResultItemConnection, error) {
	var err error

	limit := queries.Limit(first)
	log.Printf("resolvers: searching %s for %d items: %s", topic, limit, searchString)
	query := queries.NewSearch(topic, &searchString)

	topics, err := query.DescendantTopics(ctx, r.DB, limit)
	if err != nil {
		return nil, err
	}
	log.Printf("resolvers: found %d topics, with a requested limit of %d", len(topics), limit)

	if limit < len(topics) {
		limit = 0
	} else {
		limit -= len(topics)
	}

	links, err := query.DescendantLinks(ctx, r.DB, limit)
	if err != nil {
		return nil, err
	}
	log.Printf("resolvers: found %d links, with a requested limit of %d", len(links), limit)

	edges := make([]*models.SearchResultItemEdge, len(topics)+len(links))
	for i, t := range topics {
		topicValue := models.TopicValue{Topic: t, NewlyAdded: false, View: topic.View}
		edges[i] = &models.SearchResultItemEdge{Node: topicValue}
	}
	linkStart := len(topics)
	for i, l := range links {
		linkValue := models.LinkValue{Link: l, NewlyAdded: false, View: topic.View}
		edges[i+linkStart] = &models.SearchResultItemEdge{Node: linkValue}
	}

	log.Printf("resolvers: search within %s complete, returning %d results", topic, len(edges))
	return &models.SearchResultItemConnection{Edges: edges}, nil
}

// Synonyms return the synonyms for this topic.
func (r *topicResolver) Synonyms(ctx context.Context, topic *models.TopicValue) ([]*models.Synonym, error) {
	synonyms, err := topic.SynonymList()
	if err != nil {
		return nil, errors.Wrap(err, "resolvers: failed to fetch synonym list")
	}

	var out []*models.Synonym
	for _, synonym := range synonyms.Values {
		out = append(out, &models.Synonym{Locale: synonym.Locale, Name: synonym.Name})
	}

	return out, nil
}

// TimeRange returns a time range associated with the topic, if one exists.
func (r *topicResolver) TimeRange(ctx context.Context, topic *models.TopicValue) (*models.TimeRange, error) {
	timerange, err := queries.TimeRange(ctx, r.DB, topic.Topic)
	if err != nil {
		return nil, errors.Wrap(err, "resolvers: failed to fetch time range")
	}

	if timerange == nil || timerange.StartsAt.IsZero() {
		return nil, nil
	}

	format := models.TimeRangePrefixFormat(timerange.PrefixFormat)

	return &models.TimeRange{
		StartsAt:     timerange.StartsAt.Format(time.RFC3339),
		PrefixFormat: format,
	}, nil
}

// UpdatedAt returns the time of the most recent update.
func (r *topicResolver) UpdatedAt(_ context.Context, topic *models.TopicValue) (string, error) {
	return topic.UpdatedAt.Format(time.RFC3339), nil
}

func (r *topicResolver) ViewerCanUpdate(ctx context.Context, topic *models.TopicValue) (bool, error) {
	viewer := GetRequestContext(ctx).Viewer()

	if topic.Root || viewer.IsGuest() {
		return false, nil
	}

	repo := topic.GetRepo()
	if repo == nil {
		return false, nil
	}

	owner := repo.GetOwner()
	if owner == nil {
		return false, nil
	}

	if repo.IsPrivate() && owner.ID != viewer.ID {
		return false, nil
	}

	return true, nil
}

func (r *topicResolver) ViewerCanDeleteSynonyms(ctx context.Context, topic *models.TopicValue) (bool, error) {
	synonyms, err := topic.SynonymList()
	if err != nil {
		return false, err
	}

	if len(synonyms.Values) < 2 {
		return false, nil
	}

	return r.ViewerCanUpdate(ctx, topic)
}
