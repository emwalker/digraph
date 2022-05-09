package resolvers

import (
	"context"

	"github.com/emwalker/digraph/golang/internal/models"
)

type synonymResolver struct{ *Resolver }

func (r *synonymResolver) Locale(ctx context.Context, synonym *models.Synonym) (models.LocaleIdentifier, error) {
	return models.LocaleIdentifier(synonym.Locale), nil
}
