package resolvers

import (
	"context"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

var defaultSynonymSortOrder = qm.OrderBy("locale <> 'en', synonyms.created_at asc")

type synonymResolver struct{ *Resolver }

func synonymConnection(synonyms []*models.Synonym) *models.SynonymConnection {
	edges := make([]*models.SynonymEdge, len(synonyms))

	for i, synonym := range synonyms {
		edges[i] = &models.SynonymEdge{Node: *synonym}
	}

	return &models.SynonymConnection{Edges: edges}
}

// Email returns the email of a user.
func (r *synonymResolver) Locale(_ context.Context, syn *models.Synonym) (string, error) {
	return syn.Locale, nil
}

func (r *synonymResolver) Topic(ctx context.Context, syn *models.Synonym) (models.TopicValue, error) {
	return models.TopicValue{}, nil
}
