package resolvers

import (
	"context"
	"testing"

	"github.com/emwalker/digraph/models"
	"github.com/emwalker/digraph/resolvers"
	"github.com/stretchr/testify/assert"
)

func TestView(t *testing.T) {
	testDB = newTestDb(t)
	defer testDB.Close()

	t.Run("upsertLink", testQueryView)
}

func testQueryView(t *testing.T) {
	// When the organization is in the db
	query := (&resolvers.Resolver{DB: testDB}).View()
	v1 := &models.View{OrganizationIds: []string{orgId}}
	connection, err := query.Topics(context.Background(), v1, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.NotEmpty(t, connection.Edges)

	// When the organization is not in the db
	fakeId := "542d7ecc-f378-11e8-8eb2-f2801f1b9fd1"
	v2 := &models.View{OrganizationIds: []string{fakeId}}
	connection, err = query.Topics(context.Background(), v2, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.Empty(t, connection.Edges)

	// When no organization id is provided
	v3 := &models.View{OrganizationIds: []string{}}
	connection, err = query.Topics(context.Background(), v3, nil, nil, nil, nil)
	if !assert.Nil(t, err) {
		return
	}

	assert.Empty(t, connection.Edges)
}
