package resolvers_test

import (
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/stretchr/testify/assert"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

func (m mutator) deleteTopic(topic models.Topic) {
	count, err := topic.Delete(m.ctx, m.db)
	if assert.Nil(m.t, err) {
		assert.Equal(m.t, int64(1), count)
	}
}

func (m mutator) createTopic(name string) (*models.CreateTopicPayload, func()) {
	parentTopic, err := models.Topics(qm.Where("name like 'Everything'")).One(m.ctx, m.db)
	assert.Nil(m.t, err)
	assert.NotNil(m.t, parentTopic)

	input := models.CreateTopicInput{
		Name:           name,
		OrganizationID: orgId,
		TopicIds:       []string{parentTopic.ID},
	}

	var p1 *models.CreateTopicPayload
	p1, err = m.resolver.CreateTopic(m.ctx, input)
	assert.Nil(m.t, err)
	assert.NotNil(m.t, p1)

	cleanup := func() {
		m.deleteTopic(p1.TopicEdge.Node)
	}

	return p1, cleanup
}

func (m mutator) addParentTopicToTopic(child, parent models.Topic) {
	everything, err := models.Topics(qm.Where("name like 'Everything'")).One(m.ctx, testDB)
	if err != nil {
		m.t.Fatal(err)
	}

	input := models.UpdateTopicParentTopicsInput{
		TopicID:        child.ID,
		ParentTopicIds: []string{everything.ID, parent.ID},
	}

	if _, err := m.resolver.UpdateTopicParentTopics(m.ctx, input); err != nil {
		m.t.Fatal(err)
	}
}

func TestCreateTopic(t *testing.T) {
	m := newMutator(t)

	p1, cleanup := m.createTopic("Agriculture")
	defer cleanup()

	topic := p1.TopicEdge.Node

	parent, err := topic.ParentTopics().One(m.ctx, testDB)
	assert.Nil(t, err)
	assert.NotNil(t, parent)
}

func TestUpdateTopic(t *testing.T) {
	m := newMutator(t)

	p1, cleanup := m.createTopic("Agriculture")
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

	p2, err := m.resolver.UpdateTopic(m.ctx, input)

	if !assert.Nil(t, err) {
		return
	}

	assert.Equal(t, topic.ID, p2.Topic.ID)

	topic = p2.Topic
	err = topic.Reload(m.ctx, m.db)
	assert.Nil(t, err)
	assert.Equal(t, "Agricultures", topic.Name)
}

func TestTopicParentTopics(t *testing.T) {
	m := newMutator(t)

	p1, cleanup := m.createTopic("Agriculture")
	defer cleanup()
	topic1 := p1.TopicEdge.Node

	p2, cleanup := m.createTopic("Crop rotation")
	defer cleanup()
	topic2 := p2.TopicEdge.Node

	parentTopics, err := topic2.ParentTopics().All(m.ctx, m.db)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(parentTopics))

	m.addParentTopicToTopic(topic2, topic1)

	if parentTopics, err = topic2.ParentTopics().All(m.ctx, m.db); err != nil {
		t.Fatal(err)
	}
	assert.Equal(t, 2, len(parentTopics))
}

func TestSearchChildTopics(t *testing.T) {
	m := newMutator(t)

	p1, cleanup := m.createTopic("Agriculture")
	defer cleanup()
	topic := p1.TopicEdge.Node

	p2, cleanup := m.createTopic("Crop rotation")
	defer cleanup()
	childTopic := p2.TopicEdge.Node

	m.addParentTopicToTopic(childTopic, topic)

	cases := []struct {
		Name         string
		SearchString string
		Count        int
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "",
			Count:        1,
		},
		{
			Name:         "When there is a full match",
			SearchString: "crop rotation",
			Count:        1,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "crop rota",
			Count:        1,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "rotation",
			Count:        0, // Maybe later
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
		},
	}

	topicResolver := (&resolvers.Resolver{DB: testDB}).Topic()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := topicResolver.ChildTopics(m.ctx, &topic, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); td.Count != count {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}
