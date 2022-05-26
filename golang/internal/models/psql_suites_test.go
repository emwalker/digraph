// Code generated by SQLBoiler 4.11.0 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
// This file is meant to be re-generated in place and/or deleted at any time.

package models

import "testing"

func TestUpsert(t *testing.T) {
	t.Run("DeletedUsers", testDeletedUsersUpsert)

	t.Run("GithubAccounts", testGithubAccountsUpsert)

	t.Run("GoogleAccounts", testGoogleAccountsUpsert)

	t.Run("LinkTransitiveClosures", testLinkTransitiveClosuresUpsert)

	t.Run("Links", testLinksUpsert)

	t.Run("OrganizationMembers", testOrganizationMembersUpsert)

	t.Run("Organizations", testOrganizationsUpsert)

	t.Run("Repositories", testRepositoriesUpsert)

	t.Run("SchemaMigrations", testSchemaMigrationsUpsert)

	t.Run("Sessions", testSessionsUpsert)

	t.Run("Timeranges", testTimerangesUpsert)

	t.Run("TopicTransitiveClosures", testTopicTransitiveClosuresUpsert)

	t.Run("Topics", testTopicsUpsert)

	t.Run("UserLinkReviews", testUserLinkReviewsUpsert)

	t.Run("UserLinkTopics", testUserLinkTopicsUpsert)

	t.Run("UserLinks", testUserLinksUpsert)

	t.Run("Users", testUsersUpsert)
}
