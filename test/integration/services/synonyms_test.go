package services_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestAddSynonym(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "A topic", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer topicResult.Cleanup()
	topic := topicResult.Topic

	synonymName := "98b56639"
	synResult, err := c.AddSynonym(ctx, topic, synonymName, "en")
	if err != nil {
		t.Fatal(err)
	}
	defer synResult.Cleanup()

	count, err := topic.Synonyms(qm.Where("name = ?", topic.Name)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count != 1 {
		t.Fatal("Expected initial synonym to still be around")
	}

	count, err = topic.Synonyms(qm.Where("name = ?", synonymName)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count != 1 {
		t.Fatal("Expected new synonym to be created")
	}
}

func TestAddBlankSynonym(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "A topic", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer topicResult.Cleanup()

	synResult, err := c.AddSynonym(ctx, topicResult.Topic, "", "en")
	if err != nil {
		t.Fatal(err)
	}

	if synResult.Synonym != nil {
		t.Fatal("Should not have created a synonym")
	}

	if len(synResult.Alerts) < 1 {
		t.Fatal("Should have included an alert")
	}
}

func TestDeleteSynonym(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()
	synonymName := "2e970f7f88"

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "A topic", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer topicResult.Cleanup()
	topic := topicResult.Topic

	synResult, err := c.AddSynonym(ctx, topic, synonymName, "en")
	if err != nil {
		t.Fatal(err)
	}

	delResult, err := c.DeleteSynonym(ctx, synResult.Synonym)
	if err != nil {
		t.Fatal(err)
	}

	if !delResult.Success {
		t.Fatal("Expected to delete synonym")
	}

	count, err := topic.Synonyms(qm.Where("name = ?", synonymName)).Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count != 0 {
		t.Fatalf("Expected there to be no synonym: %s", synonymName)
	}
}

func TestDeleteSynonymFailsIfLastSynonym(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()
	synonymName := "2e970f7f88"

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "A topic", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer topicResult.Cleanup()
	topic := topicResult.Topic

	synResult, err := c.AddSynonym(ctx, topic, synonymName, "en")
	if err != nil {
		t.Fatal(err)
	}

	_, err = synResult.Synonym.Delete(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	synonym, err := topic.Synonyms(qm.Where("name = ?", topic.Name)).One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	delResult, err := c.DeleteSynonym(ctx, synonym)
	if err != nil {
		t.Fatal(err)
	}

	if delResult.Success {
		t.Fatal("Deletion should not have succeeded")
	}

	if len(delResult.Alerts) < 1 {
		t.Fatal("An alert should have been created")
	}
}
