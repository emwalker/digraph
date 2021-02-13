package services_test

import (
	"context"
	"reflect"
	"testing"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	in "github.com/emwalker/digraph/test/integration"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestUpsertTopicEnsuresATopic(t *testing.T) {
	ctx := context.Background()
	in.NewMutator(in.MutatorOptions{}).DeleteTopicsByName("62ce187241e")

	c := services.Connection{Exec: in.DB, Actor: in.Actor}

	result, err := c.UpsertTopic(ctx, in.Repository, "62ce187241e", nil, []string{})
	in.Must(err)

	if result.TopicCreated != true {
		t.Fatal("Expected topic to be a new one")
	}

	topics, err := result.Topic.ParentTopics().All(ctx, in.DB)
	in.Must(err)

	if len(topics) < 1 {
		t.Fatal("Expected the topic to be added to the root topic")
	}
}

func TestDisallowEmptyTopic(t *testing.T) {
	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, in.Repository, "  ", nil, []string{})
	in.Must(err)

	if result.TopicCreated {
		t.Fatal("An empty topic should not be created")
	}

	if len(result.Alerts) < 1 {
		t.Fatal("There should be an alert")
	}
}

func TestSynonymCreated(t *testing.T) {
	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()
	name := "a0257068ede"

	result, err := c.UpsertTopic(ctx, in.Repository, name, nil, []string{})
	in.Must(err)

	synonyms, err := result.Topic.SynonymList()
	in.Must(err)

	if len(synonyms.Values) != 1 {
		t.Fatalf("Expected a single synonym to be created, found %v", synonyms)
	}
}

func TestUpdateSynonyms(t *testing.T) {
	ctx := context.Background()
	in.NewMutator(in.MutatorOptions{}).DeleteTopicsByName("Backhoe")

	c := services.Connection{Exec: in.DB, Actor: in.Actor}

	result, err := c.UpsertTopic(ctx, in.Repository, "Backhoe", nil, []string{})
	in.Must(err)

	topic := result.Topic

	initialExpected := &models.SynonymList{
		Values: []models.Synonym{
			{Locale: "en", Name: "Backhoe"},
		},
	}

	initialActual, err := topic.SynonymList()
	in.Must(err)

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

	err = topic.Reload(ctx, in.DB)
	in.Must(err)

	finalActual, err := topic.SynonymList()
	in.Must(err)

	if !reflect.DeepEqual(finalActual, finalExpected) {
		t.Fatalf("Expected %#v, got %#v", finalExpected, finalActual)
	}
}

func TestDeduplicationOfSynonyms(t *testing.T) {
	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, in.Repository, "Backhoe", nil, []string{})
	in.Must(err)

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

	_, err = c.UpdateSynonyms(ctx, topic, synonyms)
	in.Must(err)

	err = topic.Reload(ctx, in.DB)
	in.Must(err)

	actual, err := topic.SynonymList()
	in.Must(err)

	if !reflect.DeepEqual(actual, expected) {
		t.Fatalf("Expected %#v, got %#v", expected, actual)
	}
}

func TestNormalizationOfSynonyms(t *testing.T) {
	c := services.Connection{Exec: in.DB, Actor: in.Actor}
	ctx := context.Background()

	result, err := c.UpsertTopic(ctx, in.Repository, "Backhoe", nil, []string{})
	in.Must(err)

	topic := result.Topic

	synonyms := []models.Synonym{
		{Locale: "en", Name: "Backhoe"},
		{Locale: "en", Name: " Excavator "},
	}

	// Synonyms are cleaned up
	normalizedSynonyms := []models.Synonym{
		{Locale: "en", Name: "Backhoe"},
		{Locale: "en", Name: "Excavator"},
	}

	expected := &models.SynonymList{Values: normalizedSynonyms}

	_, err = c.UpdateSynonyms(ctx, topic, synonyms)
	in.Must(err)

	err = topic.Reload(ctx, in.DB)
	in.Must(err)

	actual, err := topic.SynonymList()
	in.Must(err)

	if !reflect.DeepEqual(actual, expected) {
		t.Fatalf("Expected %#v, got %#v", expected, actual)
	}
}

func TestUpsertTopicTimeRange(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})
	mutator.DeleteTopicsByName("William invades England")
	mutator.DeleteTopicsByName("1066 William invades England")

	c := services.Connection{Exec: in.DB, Actor: in.Actor}

	topicResult, err := c.UpsertTopic(ctx, in.Repository, "William invades England", nil, []string{})
	in.Must(err)

	topic := topicResult.Topic

	count, err := topic.Timerange().Count(ctx, in.DB)
	in.Must(err)

	if count > 0 {
		t.Fatal("Expected no timeline")
	}

	t1 := time.Date(1066, time.October, 14, 0, 0, 0, 0, time.UTC)
	result, err := c.UpsertTopicTimeRange(ctx, topic, t1, nil, models.TimeRangePrefixFormatStartYear)
	in.Must(err)

	if result.TimeRange == nil {
		t.Fatal("Expected a time range to be returned")
	}

	timerange, err := topic.Timerange().One(ctx, in.DB)
	in.Must(err)

	if timerange == nil {
		t.Fatal("Expected a timeline")
	}

	in.Must(topic.Reload(ctx, in.DB))

	synonyms, err := topic.SynonymList()
	in.Must(err)

	displayName, err := services.DisplayName(timerange, synonyms, models.LocaleIdentifierEn)
	in.Must(err)

	expectedDisplayName := "1066 William invades England"

	if displayName != expectedDisplayName {
		t.Fatalf("Expected %s, got %s", expectedDisplayName, displayName)
	}

	// The topic name should be updated as well.  The topic name will eventually go away, but for
	// the moment it's used for sorting topics, which should take into account the time range prefix.
	in.Must(topic.Reload(ctx, in.DB))

	if topic.Name != expectedDisplayName {
		t.Fatalf("Expected %s, got %s", expectedDisplayName, topic.Name)
	}
}

func TestDeleteTopicFailsForRoot(t *testing.T) {
	ctx := context.Background()
	topics, err := models.Topics(qm.Where("root")).All(ctx, in.DB)
	in.Must(err)

	if len(topics) < 1 {
		t.Fatal("Expected at least one root topic")
	}

	for _, topic := range topics {
		if !topic.Root {
			t.Fatalf("Expected a root topic: %s", topic)
		}

		service := services.DeleteTopic{Topic: topic, Actor: in.Actor}
		result, err := service.Call(ctx, in.DB)
		if err == nil {
			t.Fatal("There should have been an error")
		}

		if result != nil {
			t.Fatal("A result should not be returned")
		}
	}
}

func TestUpdateTopicParentTopics(t *testing.T) {
	ctx := context.Background()
	mutator := in.NewMutator(in.MutatorOptions{})

	parent1 := mutator.UpsertTopic(in.UpsertTopicOptions{Name: "Parent 1"})
	parent2 := mutator.UpsertTopic(in.UpsertTopicOptions{Name: "Parent 2"})
	child := mutator.UpsertTopic(in.UpsertTopicOptions{
		Name:           "Child",
		ParentTopicIds: []string{parent1.ID, parent2.ID},
	})

	service := services.UpdateTopicParentTopics{
		Actor:          in.Actor,
		Topic:          child,
		ParentTopicIds: []string{parent1.ID},
	}
	_, err := service.Call(ctx, in.DB)
	in.Must(err)

	parentTopics, err := child.ParentTopics().All(ctx, in.DB)
	in.Must(err)

	if len(parentTopics) != 1 {
		t.Fatalf("Expected 1 parent topic for %s, got %d", child, len(parentTopics))
	}
}
