package services_test

import (
	"context"
	"reflect"
	"testing"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertTopicEnsuresATopic(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "62ce187241e", nil, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer result.Cleanup()

	if result.TopicCreated != true {
		t.Fatal("Expected topic to be a new one")
	}

	topics, err := result.Topic.ParentTopics().All(ctx, c.Exec)
	if err != nil {
		t.Fatalf("Unable to fetch parent topics: %s", err)
	}

	if len(topics) < 1 {
		t.Fatal("Expected the topic to be added to the root topic")
	}
}

func TestDisallowEmptyTopic(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "  ", nil, []string{})
	if err != nil {
		t.Fatalf("There was a problem upserting the topic: %s", err)
	}
	defer result.Cleanup()

	if result.TopicCreated {
		t.Fatal("An empty topic should not be created")
	}

	if len(result.Alerts) < 1 {
		t.Fatal("There should be an alert")
	}
}

func TestSynonymCreated(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()
	name := "a0257068ede"

	result, err := c.UpsertTopic(ctx, defaultRepo, name, nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	synonyms, err := result.Topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if len(synonyms.Values) != 1 {
		t.Fatalf("Expected a single synonym to be created, found %v", synonyms)
	}
}

func TestUpdateSynonyms(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "Backhoe", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	topic := result.Topic

	initialExpected := &models.SynonymList{
		Values: []models.Synonym{
			{Locale: "en", Name: "Backhoe"},
		},
	}

	initialActual, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if !reflect.DeepEqual(initialActual, initialExpected) {
		t.Fatalf("Expected %#v, got %#v", initialExpected, initialActual)
	}

	synonyms := []models.Synonym{
		{Locale: "en", Name: "Backhoe"},
		{Locale: "en", Name: "Excavator"},
		{Locale: "en", Name: "Grader"},
	}

	finalExpected := &models.SynonymList{Values: synonyms}

	if _, err = c.UpdateSynonyms(ctx, topic, synonyms); err != nil {
		t.Fatal(err)
	}

	if err = topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	finalActual, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if !reflect.DeepEqual(finalActual, finalExpected) {
		t.Fatalf("Expected %#v, got %#v", finalExpected, finalActual)
	}
}

func TestDeduplicationOfSynonyms(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, defaultRepo, "Backhoe", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer result.Cleanup()

	topic := result.Topic

	synonyms := []models.Synonym{
		{Locale: "en", Name: "Backhoe"},
		{Locale: "en", Name: "Backhoe"},
		{Locale: "fr", Name: "Backhoe"},
		{Locale: "en", Name: "Excavator"},
		{Locale: "en", Name: "Backhoe"},
	}

	dedupedSynonyms := []models.Synonym{
		{Locale: "en", Name: "Backhoe"},
		{Locale: "fr", Name: "Backhoe"},
		{Locale: "en", Name: "Excavator"},
	}

	expected := &models.SynonymList{Values: dedupedSynonyms}

	if _, err = c.UpdateSynonyms(ctx, topic, synonyms); err != nil {
		t.Fatal(err)
	}

	if err = topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	actual, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	if !reflect.DeepEqual(actual, expected) {
		t.Fatalf("Expected %#v, got %#v", expected, actual)
	}
}

func TestUpsertTopicTimeRange(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	topicResult, err := c.UpsertTopic(ctx, defaultRepo, "William invades England", nil, []string{})
	if err != nil {
		t.Fatal(err)
	}
	defer topicResult.Cleanup()

	topic := topicResult.Topic

	count, err := topic.Timerange().Count(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if count > 0 {
		t.Fatal("Expected no timeline")
	}

	t1 := time.Date(1066, time.October, 14, 0, 0, 0, 0, time.UTC)
	result, err := c.UpsertTopicTimeRange(ctx, topic, t1, nil, models.TimeRangePrefixFormatStartYear)

	if err != nil {
		t.Fatal(err)
	}

	if result.TimeRange == nil {
		t.Fatal("Expected a time range to be returned")
	}

	timerange, err := topic.Timerange().One(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if timerange == nil {
		t.Fatal("Expected a timeline")
	}

	if err = topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	synonyms, err := topic.SynonymList()
	if err != nil {
		t.Fatal(err)
	}

	displayName, err := services.DisplayName(timerange, synonyms, models.LocaleIdentifierEn)
	if err != nil {
		t.Fatal(err)
	}

	expectedDisplayName := "1066 William invades England"

	if displayName != expectedDisplayName {
		t.Fatalf("Expected %s, got %s", expectedDisplayName, displayName)
	}

	// The topic name should be updated as well.  The topic name will eventually go away, but for
	// the moment it's used for sorting topics, which should take into account the time range prefix.

	if err = topic.Reload(ctx, testDB); err != nil {
		t.Fatal(err)
	}

	if topic.Name != expectedDisplayName {
		t.Fatalf("Expected %s, got %s", expectedDisplayName, topic.Name)
	}
}

func TestDeleteTopicFailsForRoot(t *testing.T) {
	c := services.Connection{Exec: testDB, Actor: testActor}
	ctx := context.Background()

	topics, err := models.Topics(qm.Where("root")).All(ctx, testDB)
	if err != nil {
		t.Fatal(err)
	}

	if len(topics) < 1 {
		t.Fatal("Expected at least one root topic")
	}

	for _, topic := range topics {
		if !topic.Root {
			t.Fatalf("Expected a root topic: %s", topic)
		}

		result, err := c.DeleteTopic(ctx, topic)
		if err == nil {
			t.Fatal("Expected an error")
		}

		if result != nil {
			t.Fatal("A result should not be returned")
		}
	}
}
