package resolvers

import (
	"context"
	"log"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type viewResolver struct{ *Resolver }

func newViewFromTopic(ctx context.Context, exec boil.ContextExecutor, topic *models.Topic) (*models.View, error) {
	repo, err := fetchRepository(ctx, topic.RepositoryID)
	if err != nil {
		return nil, err
	}

	org, err := fetchOrganization(ctx, repo.OrganizationID)
	if err != nil {
		return nil, err
	}

	return &models.View{
		CurrentOrganizationLogin: org.Login,
		CurrentRepositoryName:    &repo.Name,
		CurrentRepository:        &repo,
	}, nil
}

func pageSizeOrDefault(first *int) int {
	if first == nil {
		return 100
	}
	return *first
}

func topicQueryMods(view *models.View, filter qm.QueryMod, searchString *string, first *int) []qm.QueryMod {
	mods := view.Filter([]qm.QueryMod{
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on topics.repository_id = r.id"),
	})

	if filter != nil {
		mods = append(mods, filter)
	}

	if searchString != nil {
		mods = append(mods, qm.Where("topics.name ~~* all(?)", wildcardStringArray(*searchString)))
	}

	return mods
}

// Link returns a specific link.
func (r *viewResolver) Link(
	ctx context.Context, view *models.View, linkID string,
) (*models.LinkValue, error) {
	mods := view.Filter([]qm.QueryMod{
		qm.Where("links.id = ?", linkID),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	link, err := models.Links(mods...).One(ctx, r.DB)
	return &models.LinkValue{link, false, view}, err
}

// Links returns a set of links.
func (r *viewResolver) Links(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.LinkConnection, error) {
	mods := view.Filter([]qm.QueryMod{
		qm.OrderBy("created_at desc"),
		qm.Limit(pageSizeOrDefault(first)),
		qm.InnerJoin("repositories r on links.repository_id = r.id"),
	})

	if searchString != nil && *searchString != "" {
		mods = append(mods, qm.Where("title ~~* all(?)", wildcardStringArray(*searchString)))
	}

	scope := models.Links(mods...)
	conn, err := scope.All(ctx, r.DB)
	return linkConnection(view, conn, err)
}

func (r *viewResolver) CurrentRepository(
	ctx context.Context, view *models.View,
) (*models.Repository, error) {
	return view.CurrentRepository, nil
}

// Topic returns a topic for a given id.
func (r *viewResolver) Topic(
	ctx context.Context, view *models.View, topicID string,
) (*models.TopicValue, error) {
	log.Printf("Fetching topic %s", topicID)
	scope := models.Topics(topicQueryMods(view, qm.Where("topics.id = ?", topicID), nil, nil)...)
	topic, err := scope.One(ctx, r.DB)
	return &models.TopicValue{topic, false, view}, err
}

// TopicGraph returns a json string that can be used for building a visual representation of the graph.
func (r *viewResolver) TopicGraph(ctx context.Context, view *models.View) (*string, error) {
	result := struct {
		Payload string
	}{}

	// TODO - search within the repositories specified in view.RepositoryIds
	err := queries.Raw(`
	with topic_ids as (
	  select (row_number() over (order by id) - 1) integer_id, id
	  from topics
	  where repository_id = $1
	)

	select jsonb_build_object('links', (
	  select jsonb_agg(a) from (
	    select tp.integer_id source, tc.integer_id target, count(distinct lt.child_id) "linkCount"
	    from topic_topics tt
	    join topics t1 on tt.parent_id = t1.id
	    join topics t2 on tt.child_id = t2.id
	    join topic_ids tp on tt.parent_id = tp.id
	    join topic_ids tc on tt.child_id = tc.id
	    left join link_topics lt on tt.child_id = lt.parent_id
	    where t1.repository_id = $1 and t2.repository_id = $1
	    group by tp.integer_id, tc.integer_id
	  ) a
	)) ||
	jsonb_build_object('nodes', (
	  select jsonb_agg(b) from (
	    select tid.integer_id id, t.name, count(distinct lt.child_id) "linkCount", count(distinct tt.child_id) "topicCount"
	    from topics t
	    join topic_ids tid on t.id = tid.id
	    left join link_topics lt on t.id = lt.parent_id
	    left join topic_topics tt on t.id = tt.parent_id
	    where t.repository_id = $1
	    group by tid.integer_id, t.name
	  ) b
	)) payload
	`, generalRepositoryID).Bind(ctx, r.DB, &result)

	if err != nil {
		log.Printf("There was a problem fetching topic graph: %s", err)
		return nil, err
	}

	return &result.Payload, nil
}

// Topics returns a set of topics.
func (r *viewResolver) Topics(
	ctx context.Context, view *models.View, searchString *string, first *int, after *string,
	last *int, before *string,
) (models.TopicConnection, error) {
	topics, err := models.Topics(topicQueryMods(view, nil, searchString, first)...).All(ctx, r.DB)
	return topicConnection(view, topics, err)
}
