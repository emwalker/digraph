package resolvers_test

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/stretchr/testify/assert"
)

func TestQueryView(t *testing.T) {
	// When the organization is in the db
	query := (&resolvers.Resolver{DB: testDB}).View()
	v1 := &models.View{OrganizationIds: []string{orgId}}
	connection, err := query.Topics(context.Background(), v1, nil, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.NotEmpty(t, connection.Edges)

	// When the organization is not in the db
	fakeId := "542d7ecc-f378-11e8-8eb2-f2801f1b9fd1"
	v2 := &models.View{OrganizationIds: []string{fakeId}}
	connection, err = query.Topics(context.Background(), v2, nil, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.Empty(t, connection.Edges)

	// When no organization id is provided
	v3 := &models.View{OrganizationIds: []string{}}
	connection, err = query.Topics(context.Background(), v3, nil, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.Empty(t, connection.Edges)
}

func TestSearchTopics(t *testing.T) {
	m := newMutator(t)

	p1, cleanup := m.createTopic("Agriculture")
	defer cleanup()
	topic := p1.TopicEdge.Node

	p2, cleanup := m.createTopic("Crop rotation")
	defer cleanup()
	childTopic := p2.TopicEdge.Node

	m.addParentTopicToTopic(childTopic, topic)

	ge := func(expected, actual int) bool {
		return actual >= expected
	}

	eq := func(expected, actual int) bool {
		return actual == expected
	}

	cases := []struct {
		Name         string
		SearchString string
		Count        int
		Success      func(int, int) bool
	}{
		{
			Name:         "When an empty string is provided",
			SearchString: "crop rotation",
			Count:        1,
			Success:      ge,
		},
		{
			Name:         "When there is a full match",
			SearchString: "crop rotation",
			Count:        1,
			Success:      eq,
		},
		{
			Name:         "When there is a prefix match",
			SearchString: "crop rota",
			Count:        1,
			Success:      eq,
		},
		{
			Name:         "When there is a suffix match",
			SearchString: "rotation",
			Count:        0, // Maybe later
			Success:      eq,
		},
		{
			Name:         "When there is no match",
			SearchString: "astronomy",
			Count:        0,
			Success:      eq,
		},
	}

	view := &models.View{OrganizationIds: []string{orgId}}
	viewResolver := (&resolvers.Resolver{DB: testDB}).View()

	for _, td := range cases {
		t.Run(td.Name, func(t *testing.T) {
			conn, err := viewResolver.Topics(m.ctx, view, &td.SearchString, nil, nil, nil, nil)
			if err != nil {
				t.Fatal(err)
			}

			if count := len(conn.Edges); !td.Success(td.Count, count) {
				t.Fatalf("Expected %d results, got %d", td.Count, count)
			}
		})
	}
}
