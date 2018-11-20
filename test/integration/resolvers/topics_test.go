package resolvers

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func deleteTopic(t *testing.T, topic models.Topic) {
	count, err := topic.Delete(context.Background(), testDB)
	if assert.Nil(t, err) {
		assert.Equal(t, int64(1), count)
	}
}

func TestTopics(t *testing.T) {
	testDB = newTestDb(t)
	defer testDB.Close()

	t.Run("createTopic", createTopicTest)
	t.Run("updateTopicTest", updateTopicTest)
}

func createTopic(t *testing.T, r models.MutationResolver, ctx context.Context) *models.CreateTopicPayload {
	parentTopic, err := models.Topics(qm.Where("name like 'Science'")).One(ctx, testDB)
	assert.Nil(t, err)
	assert.NotNil(t, parentTopic)

	desc := "Cultivation of the earth for food"
	input := models.CreateTopicInput{
		Description:    &desc,
		Name:           "Agriculture",
		OrganizationID: orgId,
		TopicIds:       []string{parentTopic.ID},
	}

	var payload *models.CreateTopicPayload
	payload, err = r.CreateTopic(ctx, input)
	assert.Nil(t, err)
	assert.NotNil(t, payload)
	return payload
}

func createTopicTest(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	payload := createTopic(t, r, ctx)
	topic := payload.TopicEdge.Node

	parent, err := topic.ParentTopics().One(ctx, testDB)
	assert.Nil(t, err)
	assert.NotNil(t, parent)

	deleteTopic(t, topic)
}

func updateTopicTest(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	payload1 := createTopic(t, r, ctx)
	topic := payload1.TopicEdge.Node
	assert.Equal(t, "Agriculture", topic.Name)

	var err error
	desc := "Cultivating"

	input := models.UpdateTopicInput{
		Name:           "Agricultures",
		Description:    &desc,
		OrganizationID: orgId,
		ID:             topic.ID,
	}

	payload2, err := r.UpdateTopic(ctx, input)

	if assert.Nil(t, err) {
		assert.Equal(t, topic.ID, payload2.Topic.ID)

		topic = payload2.Topic
		err = topic.Reload(ctx, testDB)
		assert.Nil(t, err)
		assert.Equal(t, "Agricultures", topic.Name)

		deleteTopic(t, topic)
	}
}
