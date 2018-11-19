package resolvers

import (
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func TestTopics(t *testing.T) {
	testDB = newTestDb(t)
	defer testDB.Close()

	t.Run("createTopic", createTopicTest)
}

func createTopicTest(t *testing.T) {
	r, ctx, tx := startMutationTest(t, testDB)
	defer tx.Rollback()

	parentTopic, err := models.Topics(qm.Where("name like 'Science'")).One(ctx, tx)
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

	topic := payload.TopicEdge.Node
	var parent *models.Topic

	parent, err = topic.ParentTopics().One(ctx, tx)
	assert.Nil(t, err)
	assert.NotNil(t, parent)
}
