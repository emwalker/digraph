package resolvers_test

import (
	"context"
	"fmt"
	"testing"

	"github.com/emwalker/digraph/resolvers"
)

func TestRootTopic(t *testing.T) {
	ctx := context.Background()
	resolver := &resolvers.Resolver{DB: testDB}

	defaultRepo, err := testActor.DefaultRepo(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	rootTopic, err := defaultRepo.RootTopic(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	path := fmt.Sprintf("/%s/topics/%s", testActor.Login, rootTopic.ID)
	repoResolver := resolver.Repository()

	topic, err := repoResolver.RootTopic(context.Background(), defaultRepo)
	if err != nil {
		t.Fatal(err)
	}

	topicResolver := resolver.Topic()
	var rootPath string

	if rootPath, err = topicResolver.ResourcePath(ctx, &topic); err != nil {
		t.Fatal(err)
	}

	if path != rootPath {
		t.Fatalf("Unexpected root path: %s", rootPath)
	}
}
