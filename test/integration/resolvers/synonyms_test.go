package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/models"
)

func TestAddSynonym(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
	defer cleanup()

	input := models.AddSynonymInput{
		Name:    "50d9b71d650ffd",
		TopicID: topic.ID,
	}

	payload, err := m.resolver.AddSynonym(m.ctx, input)
	if err != nil {
		t.Fatal(err)
	}

	if len(payload.Alerts) > 0 {
		t.Fatal("There should be no alerts")
	}

	if payload.Topic.ID != topic.ID {
		t.Fatal("Expected the topic to be returned")
	}
}

func TestDeleteSynonym(t *testing.T) {
	m := newMutator(t, testActor)
	repoName := m.defaultRepo().Name

	topic, cleanup := m.createTopic(testActor.Login, repoName, "Agriculture")
	defer cleanup()

	addInput := models.AddSynonymInput{
		Locale:  "en",
		Name:    "50d9b71d650ffd",
		TopicID: topic.ID,
	}

	addPayload, err := m.resolver.AddSynonym(m.ctx, addInput)
	if err != nil {
		t.Fatal(err)
	}

	delInput := models.DeleteSynonymInput{SynonymID: addPayload.SynonymEdge.Node.ID}

	delPayload, err := m.resolver.DeleteSynonym(m.ctx, delInput)
	if err != nil {
		t.Fatal(err)
	}

	if len(delPayload.Alerts) > 0 {
		t.Fatalf("There should be no alerts: %#v", delPayload.Alerts)
	}
}
