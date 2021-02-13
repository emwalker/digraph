package loaders

import (
	"context"
	"time"

	"github.com/volatiletech/sqlboiler/boil"
)

type config struct {
	exec boil.ContextExecutor
}

func convertIds(ids []string) []interface{} {
	var translatedIds []interface{}
	for _, id := range ids {
		translatedIds = append(translatedIds, id)
	}
	return translatedIds
}

func AddToContext(ctx context.Context, exec boil.ContextExecutor, wait time.Duration) context.Context {
	ctx = context.WithValue(ctx, TopicLoaderKey, newTopicLoader(ctx, exec, wait))
	ctx = context.WithValue(ctx, OrganizationLoaderKey, newOrganizationLoader(ctx, exec, wait))
	ctx = context.WithValue(ctx, RepositoryLoaderKey, newRepositoryLoader(ctx, exec, wait))
	return ctx
}
