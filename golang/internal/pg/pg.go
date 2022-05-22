package pg

import (
	"log"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/util"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
	"github.com/volatiletech/sqlboiler/v4/types"
	"golang.org/x/net/context"
)

type Repo struct {
	db boil.ContextExecutor
}

func NewRepo(db boil.ContextExecutor) *Repo {
	return &Repo{db: db}
}

func (r *Repo) GetTopicsByID(ctx context.Context, ids []string) ([]*models.Topic, error) {
	log.Printf("pg: fetching topic ids: %s", ids)
	boilIDs := util.StringsToInterfaces(ids)

	topics, err := models.Topics(
		qm.WhereIn("id = any(?)", types.Array(boilIDs)),
		qm.Load("ParentTopics"),
	).All(ctx, r.db)

	if err != nil {
		return nil, err
	}
	return topics, nil
}

func (r *Repo) GetOrgsByID(ctx context.Context, ids []string) ([]*models.Organization, error) {
	log.Printf("pg: fetching organizations: %s", ids)
	boilIDs := util.StringsToInterfaces(ids)

	orgs, err := models.Organizations(
		qm.WhereIn("id = any(?)", types.Array(boilIDs)),
	).All(ctx, r.db)

	if err != nil {
		return nil, err
	}
	return orgs, nil
}

func (r *Repo) GetReposByID(ctx context.Context, ids []string) ([]*models.Repository, error) {
	log.Printf("pg: fetching repositories: %s", ids)
	boilIDs := util.StringsToInterfaces(ids)
	return models.Repositories(
		qm.WhereIn("id = any(?)", types.Array(boilIDs)),
	).All(ctx, r.db)
}

func (r *Repo) GetDefaultReposByOrgID(ctx context.Context, ids []string) ([]*models.Repository, error) {
	log.Printf("pg: fetching default repositories: %s", ids)
	boilIDs := util.StringsToInterfaces(ids)
	return models.Repositories(
		qm.Where("system and organization_id = any(?)", types.Array(boilIDs)),
	).All(ctx, r.db)
}
