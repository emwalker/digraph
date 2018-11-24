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
	t.Run("updateTopicParentTopicsTest", updateTopicParentTopicsTest)
}

func createTopic(
	t *testing.T, r models.MutationResolver, ctx context.Context,
) (*models.CreateTopicPayload, func()) {
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

	var p1 *models.CreateTopicPayload
	p1, err = r.CreateTopic(ctx, input)
	assert.Nil(t, err)
	assert.NotNil(t, p1)

	cleanup := func() {
		deleteTopic(t, p1.TopicEdge.Node)
	}

	return p1, cleanup
}

func createTopicTest(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	p1, cleanup := createTopic(t, r, ctx)
	defer cleanup()

	topic := p1.TopicEdge.Node

	parent, err := topic.ParentTopics().One(ctx, testDB)
	assert.Nil(t, err)
	assert.NotNil(t, parent)
}

func updateTopicTest(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	p1, cleanup := createTopic(t, r, ctx)
	defer cleanup()

	topic := p1.TopicEdge.Node
	assert.Equal(t, "Agriculture", topic.Name)

	var err error
	desc := "Cultivating"

	input := models.UpdateTopicInput{
		Name:           "Agricultures",
		Description:    &desc,
		OrganizationID: orgId,
		ID:             topic.ID,
	}

	p2, err := r.UpdateTopic(ctx, input)

	if !assert.Nil(t, err) {
		return
	}

	assert.Equal(t, topic.ID, p2.Topic.ID)

	topic = p2.Topic
	err = topic.Reload(ctx, testDB)
	assert.Nil(t, err)
	assert.Equal(t, "Agricultures", topic.Name)
}

func updateTopicParentTopicsTest(t *testing.T) {
	r, ctx := startMutationTest(t, testDB)

	p1, cleanup := createTopic(t, r, ctx)
	defer cleanup()
	topic1 := p1.TopicEdge.Node

	p2, cleanup := createTopic(t, r, ctx)
	defer cleanup()
	topic2 := p2.TopicEdge.Node

	parentTopics, err := topic2.ParentTopics().All(ctx, testDB)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(parentTopics))

	_, err = r.UpdateTopicParentTopics(ctx, models.UpdateTopicParentTopicsInput{
		TopicID:        topic2.ID,
		ParentTopicIds: []string{topic1.ID, parentTopics[0].ID},
	})
	if !assert.Nil(t, err) {
		return
	}

	parentTopics, err = topic2.ParentTopics().All(ctx, testDB)
	assert.Nil(t, err)
	assert.Equal(t, 2, len(parentTopics))
}
