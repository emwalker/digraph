// Code generated by SQLBoiler (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
// This file is meant to be re-generated in place and/or deleted at any time.

package models

import "testing"

// This test suite runs each operation test in parallel.
// Example, if your database has 3 tables, the suite will run:
// table1, table2 and table3 Delete in parallel
// table1, table2 and table3 Insert in parallel, and so forth.
// It does NOT run each operation group in parallel.
// Separating the tests thusly grants avoidance of Postgres deadlocks.
func TestParent(t *testing.T) {
	t.Run("Links", testLinks)
	t.Run("OrganizationMembers", testOrganizationMembers)
	t.Run("Organizations", testOrganizations)
	t.Run("Repositories", testRepositories)
	t.Run("SchemaMigrations", testSchemaMigrations)
	t.Run("Sessions", testSessions)
	t.Run("Synonyms", testSynonyms)
	t.Run("Topics", testTopics)
	t.Run("UserLinkTopics", testUserLinkTopics)
	t.Run("UserLinks", testUserLinks)
	t.Run("Users", testUsers)
}

func TestDelete(t *testing.T) {
	t.Run("Links", testLinksDelete)
	t.Run("OrganizationMembers", testOrganizationMembersDelete)
	t.Run("Organizations", testOrganizationsDelete)
	t.Run("Repositories", testRepositoriesDelete)
	t.Run("SchemaMigrations", testSchemaMigrationsDelete)
	t.Run("Sessions", testSessionsDelete)
	t.Run("Synonyms", testSynonymsDelete)
	t.Run("Topics", testTopicsDelete)
	t.Run("UserLinkTopics", testUserLinkTopicsDelete)
	t.Run("UserLinks", testUserLinksDelete)
	t.Run("Users", testUsersDelete)
}

func TestQueryDeleteAll(t *testing.T) {
	t.Run("Links", testLinksQueryDeleteAll)
	t.Run("OrganizationMembers", testOrganizationMembersQueryDeleteAll)
	t.Run("Organizations", testOrganizationsQueryDeleteAll)
	t.Run("Repositories", testRepositoriesQueryDeleteAll)
	t.Run("SchemaMigrations", testSchemaMigrationsQueryDeleteAll)
	t.Run("Sessions", testSessionsQueryDeleteAll)
	t.Run("Synonyms", testSynonymsQueryDeleteAll)
	t.Run("Topics", testTopicsQueryDeleteAll)
	t.Run("UserLinkTopics", testUserLinkTopicsQueryDeleteAll)
	t.Run("UserLinks", testUserLinksQueryDeleteAll)
	t.Run("Users", testUsersQueryDeleteAll)
}

func TestSliceDeleteAll(t *testing.T) {
	t.Run("Links", testLinksSliceDeleteAll)
	t.Run("OrganizationMembers", testOrganizationMembersSliceDeleteAll)
	t.Run("Organizations", testOrganizationsSliceDeleteAll)
	t.Run("Repositories", testRepositoriesSliceDeleteAll)
	t.Run("SchemaMigrations", testSchemaMigrationsSliceDeleteAll)
	t.Run("Sessions", testSessionsSliceDeleteAll)
	t.Run("Synonyms", testSynonymsSliceDeleteAll)
	t.Run("Topics", testTopicsSliceDeleteAll)
	t.Run("UserLinkTopics", testUserLinkTopicsSliceDeleteAll)
	t.Run("UserLinks", testUserLinksSliceDeleteAll)
	t.Run("Users", testUsersSliceDeleteAll)
}

func TestExists(t *testing.T) {
	t.Run("Links", testLinksExists)
	t.Run("OrganizationMembers", testOrganizationMembersExists)
	t.Run("Organizations", testOrganizationsExists)
	t.Run("Repositories", testRepositoriesExists)
	t.Run("SchemaMigrations", testSchemaMigrationsExists)
	t.Run("Sessions", testSessionsExists)
	t.Run("Synonyms", testSynonymsExists)
	t.Run("Topics", testTopicsExists)
	t.Run("UserLinkTopics", testUserLinkTopicsExists)
	t.Run("UserLinks", testUserLinksExists)
	t.Run("Users", testUsersExists)
}

func TestFind(t *testing.T) {
	t.Run("Links", testLinksFind)
	t.Run("OrganizationMembers", testOrganizationMembersFind)
	t.Run("Organizations", testOrganizationsFind)
	t.Run("Repositories", testRepositoriesFind)
	t.Run("SchemaMigrations", testSchemaMigrationsFind)
	t.Run("Sessions", testSessionsFind)
	t.Run("Synonyms", testSynonymsFind)
	t.Run("Topics", testTopicsFind)
	t.Run("UserLinkTopics", testUserLinkTopicsFind)
	t.Run("UserLinks", testUserLinksFind)
	t.Run("Users", testUsersFind)
}

func TestBind(t *testing.T) {
	t.Run("Links", testLinksBind)
	t.Run("OrganizationMembers", testOrganizationMembersBind)
	t.Run("Organizations", testOrganizationsBind)
	t.Run("Repositories", testRepositoriesBind)
	t.Run("SchemaMigrations", testSchemaMigrationsBind)
	t.Run("Sessions", testSessionsBind)
	t.Run("Synonyms", testSynonymsBind)
	t.Run("Topics", testTopicsBind)
	t.Run("UserLinkTopics", testUserLinkTopicsBind)
	t.Run("UserLinks", testUserLinksBind)
	t.Run("Users", testUsersBind)
}

func TestOne(t *testing.T) {
	t.Run("Links", testLinksOne)
	t.Run("OrganizationMembers", testOrganizationMembersOne)
	t.Run("Organizations", testOrganizationsOne)
	t.Run("Repositories", testRepositoriesOne)
	t.Run("SchemaMigrations", testSchemaMigrationsOne)
	t.Run("Sessions", testSessionsOne)
	t.Run("Synonyms", testSynonymsOne)
	t.Run("Topics", testTopicsOne)
	t.Run("UserLinkTopics", testUserLinkTopicsOne)
	t.Run("UserLinks", testUserLinksOne)
	t.Run("Users", testUsersOne)
}

func TestAll(t *testing.T) {
	t.Run("Links", testLinksAll)
	t.Run("OrganizationMembers", testOrganizationMembersAll)
	t.Run("Organizations", testOrganizationsAll)
	t.Run("Repositories", testRepositoriesAll)
	t.Run("SchemaMigrations", testSchemaMigrationsAll)
	t.Run("Sessions", testSessionsAll)
	t.Run("Synonyms", testSynonymsAll)
	t.Run("Topics", testTopicsAll)
	t.Run("UserLinkTopics", testUserLinkTopicsAll)
	t.Run("UserLinks", testUserLinksAll)
	t.Run("Users", testUsersAll)
}

func TestCount(t *testing.T) {
	t.Run("Links", testLinksCount)
	t.Run("OrganizationMembers", testOrganizationMembersCount)
	t.Run("Organizations", testOrganizationsCount)
	t.Run("Repositories", testRepositoriesCount)
	t.Run("SchemaMigrations", testSchemaMigrationsCount)
	t.Run("Sessions", testSessionsCount)
	t.Run("Synonyms", testSynonymsCount)
	t.Run("Topics", testTopicsCount)
	t.Run("UserLinkTopics", testUserLinkTopicsCount)
	t.Run("UserLinks", testUserLinksCount)
	t.Run("Users", testUsersCount)
}

func TestHooks(t *testing.T) {
	t.Run("Links", testLinksHooks)
	t.Run("OrganizationMembers", testOrganizationMembersHooks)
	t.Run("Organizations", testOrganizationsHooks)
	t.Run("Repositories", testRepositoriesHooks)
	t.Run("SchemaMigrations", testSchemaMigrationsHooks)
	t.Run("Sessions", testSessionsHooks)
	t.Run("Synonyms", testSynonymsHooks)
	t.Run("Topics", testTopicsHooks)
	t.Run("UserLinkTopics", testUserLinkTopicsHooks)
	t.Run("UserLinks", testUserLinksHooks)
	t.Run("Users", testUsersHooks)
}

func TestInsert(t *testing.T) {
	t.Run("Links", testLinksInsert)
	t.Run("Links", testLinksInsertWhitelist)
	t.Run("OrganizationMembers", testOrganizationMembersInsert)
	t.Run("OrganizationMembers", testOrganizationMembersInsertWhitelist)
	t.Run("Organizations", testOrganizationsInsert)
	t.Run("Organizations", testOrganizationsInsertWhitelist)
	t.Run("Repositories", testRepositoriesInsert)
	t.Run("Repositories", testRepositoriesInsertWhitelist)
	t.Run("SchemaMigrations", testSchemaMigrationsInsert)
	t.Run("SchemaMigrations", testSchemaMigrationsInsertWhitelist)
	t.Run("Sessions", testSessionsInsert)
	t.Run("Sessions", testSessionsInsertWhitelist)
	t.Run("Synonyms", testSynonymsInsert)
	t.Run("Synonyms", testSynonymsInsertWhitelist)
	t.Run("Topics", testTopicsInsert)
	t.Run("Topics", testTopicsInsertWhitelist)
	t.Run("UserLinkTopics", testUserLinkTopicsInsert)
	t.Run("UserLinkTopics", testUserLinkTopicsInsertWhitelist)
	t.Run("UserLinks", testUserLinksInsert)
	t.Run("UserLinks", testUserLinksInsertWhitelist)
	t.Run("Users", testUsersInsert)
	t.Run("Users", testUsersInsertWhitelist)
}

// TestToOne tests cannot be run in parallel
// or deadlocks can occur.
func TestToOne(t *testing.T) {
	t.Run("LinkToOrganizationUsingOrganization", testLinkToOneOrganizationUsingOrganization)
	t.Run("LinkToRepositoryUsingRepository", testLinkToOneRepositoryUsingRepository)
	t.Run("OrganizationMemberToOrganizationUsingOrganization", testOrganizationMemberToOneOrganizationUsingOrganization)
	t.Run("OrganizationMemberToUserUsingUser", testOrganizationMemberToOneUserUsingUser)
	t.Run("RepositoryToOrganizationUsingOrganization", testRepositoryToOneOrganizationUsingOrganization)
	t.Run("RepositoryToUserUsingOwner", testRepositoryToOneUserUsingOwner)
	t.Run("SessionToUserUsingUser", testSessionToOneUserUsingUser)
	t.Run("SynonymToTopicUsingTopic", testSynonymToOneTopicUsingTopic)
	t.Run("TopicToOrganizationUsingOrganization", testTopicToOneOrganizationUsingOrganization)
	t.Run("TopicToRepositoryUsingRepository", testTopicToOneRepositoryUsingRepository)
	t.Run("UserLinkTopicToUserLinkUsingUserLink", testUserLinkTopicToOneUserLinkUsingUserLink)
	t.Run("UserLinkTopicToTopicUsingTopic", testUserLinkTopicToOneTopicUsingTopic)
	t.Run("UserLinkToOrganizationUsingOrganization", testUserLinkToOneOrganizationUsingOrganization)
	t.Run("UserLinkToRepositoryUsingRepository", testUserLinkToOneRepositoryUsingRepository)
	t.Run("UserLinkToUserUsingUser", testUserLinkToOneUserUsingUser)
	t.Run("UserLinkToLinkUsingLink", testUserLinkToOneLinkUsingLink)
	t.Run("UserToRepositoryUsingSelectedRepository", testUserToOneRepositoryUsingSelectedRepository)
}

// TestOneToOne tests cannot be run in parallel
// or deadlocks can occur.
func TestOneToOne(t *testing.T) {}

// TestToMany tests cannot be run in parallel
// or deadlocks can occur.
func TestToMany(t *testing.T) {
	t.Run("LinkToParentTopics", testLinkToManyParentTopics)
	t.Run("LinkToUserLinks", testLinkToManyUserLinks)
	t.Run("OrganizationToLinks", testOrganizationToManyLinks)
	t.Run("OrganizationToOrganizationMembers", testOrganizationToManyOrganizationMembers)
	t.Run("OrganizationToRepositories", testOrganizationToManyRepositories)
	t.Run("OrganizationToTopics", testOrganizationToManyTopics)
	t.Run("OrganizationToUserLinks", testOrganizationToManyUserLinks)
	t.Run("RepositoryToLinks", testRepositoryToManyLinks)
	t.Run("RepositoryToTopics", testRepositoryToManyTopics)
	t.Run("RepositoryToUserLinks", testRepositoryToManyUserLinks)
	t.Run("RepositoryToSelectedRepositoryUsers", testRepositoryToManySelectedRepositoryUsers)
	t.Run("TopicToChildLinks", testTopicToManyChildLinks)
	t.Run("TopicToSynonyms", testTopicToManySynonyms)
	t.Run("TopicToParentTopics", testTopicToManyParentTopics)
	t.Run("TopicToChildTopics", testTopicToManyChildTopics)
	t.Run("TopicToUserLinkTopics", testTopicToManyUserLinkTopics)
	t.Run("UserLinkToUserLinkTopics", testUserLinkToManyUserLinkTopics)
	t.Run("UserToOrganizationMembers", testUserToManyOrganizationMembers)
	t.Run("UserToOwnerRepositories", testUserToManyOwnerRepositories)
	t.Run("UserToSessions", testUserToManySessions)
	t.Run("UserToUserLinks", testUserToManyUserLinks)
}

// TestToOneSet tests cannot be run in parallel
// or deadlocks can occur.
func TestToOneSet(t *testing.T) {
	t.Run("LinkToOrganizationUsingLinks", testLinkToOneSetOpOrganizationUsingOrganization)
	t.Run("LinkToRepositoryUsingLinks", testLinkToOneSetOpRepositoryUsingRepository)
	t.Run("OrganizationMemberToOrganizationUsingOrganizationMembers", testOrganizationMemberToOneSetOpOrganizationUsingOrganization)
	t.Run("OrganizationMemberToUserUsingOrganizationMembers", testOrganizationMemberToOneSetOpUserUsingUser)
	t.Run("RepositoryToOrganizationUsingRepositories", testRepositoryToOneSetOpOrganizationUsingOrganization)
	t.Run("RepositoryToUserUsingOwnerRepositories", testRepositoryToOneSetOpUserUsingOwner)
	t.Run("SessionToUserUsingSessions", testSessionToOneSetOpUserUsingUser)
	t.Run("SynonymToTopicUsingSynonyms", testSynonymToOneSetOpTopicUsingTopic)
	t.Run("TopicToOrganizationUsingTopics", testTopicToOneSetOpOrganizationUsingOrganization)
	t.Run("TopicToRepositoryUsingTopics", testTopicToOneSetOpRepositoryUsingRepository)
	t.Run("UserLinkTopicToUserLinkUsingUserLinkTopics", testUserLinkTopicToOneSetOpUserLinkUsingUserLink)
	t.Run("UserLinkTopicToTopicUsingUserLinkTopics", testUserLinkTopicToOneSetOpTopicUsingTopic)
	t.Run("UserLinkToOrganizationUsingUserLinks", testUserLinkToOneSetOpOrganizationUsingOrganization)
	t.Run("UserLinkToRepositoryUsingUserLinks", testUserLinkToOneSetOpRepositoryUsingRepository)
	t.Run("UserLinkToUserUsingUserLinks", testUserLinkToOneSetOpUserUsingUser)
	t.Run("UserLinkToLinkUsingUserLinks", testUserLinkToOneSetOpLinkUsingLink)
	t.Run("UserToRepositoryUsingSelectedRepositoryUsers", testUserToOneSetOpRepositoryUsingSelectedRepository)
}

// TestToOneRemove tests cannot be run in parallel
// or deadlocks can occur.
func TestToOneRemove(t *testing.T) {
	t.Run("UserToRepositoryUsingSelectedRepositoryUsers", testUserToOneRemoveOpRepositoryUsingSelectedRepository)
}

// TestOneToOneSet tests cannot be run in parallel
// or deadlocks can occur.
func TestOneToOneSet(t *testing.T) {}

// TestOneToOneRemove tests cannot be run in parallel
// or deadlocks can occur.
func TestOneToOneRemove(t *testing.T) {}

// TestToManyAdd tests cannot be run in parallel
// or deadlocks can occur.
func TestToManyAdd(t *testing.T) {
	t.Run("LinkToParentTopics", testLinkToManyAddOpParentTopics)
	t.Run("LinkToUserLinks", testLinkToManyAddOpUserLinks)
	t.Run("OrganizationToLinks", testOrganizationToManyAddOpLinks)
	t.Run("OrganizationToOrganizationMembers", testOrganizationToManyAddOpOrganizationMembers)
	t.Run("OrganizationToRepositories", testOrganizationToManyAddOpRepositories)
	t.Run("OrganizationToTopics", testOrganizationToManyAddOpTopics)
	t.Run("OrganizationToUserLinks", testOrganizationToManyAddOpUserLinks)
	t.Run("RepositoryToLinks", testRepositoryToManyAddOpLinks)
	t.Run("RepositoryToTopics", testRepositoryToManyAddOpTopics)
	t.Run("RepositoryToUserLinks", testRepositoryToManyAddOpUserLinks)
	t.Run("RepositoryToSelectedRepositoryUsers", testRepositoryToManyAddOpSelectedRepositoryUsers)
	t.Run("TopicToChildLinks", testTopicToManyAddOpChildLinks)
	t.Run("TopicToSynonyms", testTopicToManyAddOpSynonyms)
	t.Run("TopicToParentTopics", testTopicToManyAddOpParentTopics)
	t.Run("TopicToChildTopics", testTopicToManyAddOpChildTopics)
	t.Run("TopicToUserLinkTopics", testTopicToManyAddOpUserLinkTopics)
	t.Run("UserLinkToUserLinkTopics", testUserLinkToManyAddOpUserLinkTopics)
	t.Run("UserToOrganizationMembers", testUserToManyAddOpOrganizationMembers)
	t.Run("UserToOwnerRepositories", testUserToManyAddOpOwnerRepositories)
	t.Run("UserToSessions", testUserToManyAddOpSessions)
	t.Run("UserToUserLinks", testUserToManyAddOpUserLinks)
}

// TestToManySet tests cannot be run in parallel
// or deadlocks can occur.
func TestToManySet(t *testing.T) {
	t.Run("LinkToParentTopics", testLinkToManySetOpParentTopics)
	t.Run("RepositoryToSelectedRepositoryUsers", testRepositoryToManySetOpSelectedRepositoryUsers)
	t.Run("TopicToChildLinks", testTopicToManySetOpChildLinks)
	t.Run("TopicToParentTopics", testTopicToManySetOpParentTopics)
	t.Run("TopicToChildTopics", testTopicToManySetOpChildTopics)
}

// TestToManyRemove tests cannot be run in parallel
// or deadlocks can occur.
func TestToManyRemove(t *testing.T) {
	t.Run("LinkToParentTopics", testLinkToManyRemoveOpParentTopics)
	t.Run("RepositoryToSelectedRepositoryUsers", testRepositoryToManyRemoveOpSelectedRepositoryUsers)
	t.Run("TopicToChildLinks", testTopicToManyRemoveOpChildLinks)
	t.Run("TopicToParentTopics", testTopicToManyRemoveOpParentTopics)
	t.Run("TopicToChildTopics", testTopicToManyRemoveOpChildTopics)
}

func TestReload(t *testing.T) {
	t.Run("Links", testLinksReload)
	t.Run("OrganizationMembers", testOrganizationMembersReload)
	t.Run("Organizations", testOrganizationsReload)
	t.Run("Repositories", testRepositoriesReload)
	t.Run("SchemaMigrations", testSchemaMigrationsReload)
	t.Run("Sessions", testSessionsReload)
	t.Run("Synonyms", testSynonymsReload)
	t.Run("Topics", testTopicsReload)
	t.Run("UserLinkTopics", testUserLinkTopicsReload)
	t.Run("UserLinks", testUserLinksReload)
	t.Run("Users", testUsersReload)
}

func TestReloadAll(t *testing.T) {
	t.Run("Links", testLinksReloadAll)
	t.Run("OrganizationMembers", testOrganizationMembersReloadAll)
	t.Run("Organizations", testOrganizationsReloadAll)
	t.Run("Repositories", testRepositoriesReloadAll)
	t.Run("SchemaMigrations", testSchemaMigrationsReloadAll)
	t.Run("Sessions", testSessionsReloadAll)
	t.Run("Synonyms", testSynonymsReloadAll)
	t.Run("Topics", testTopicsReloadAll)
	t.Run("UserLinkTopics", testUserLinkTopicsReloadAll)
	t.Run("UserLinks", testUserLinksReloadAll)
	t.Run("Users", testUsersReloadAll)
}

func TestSelect(t *testing.T) {
	t.Run("Links", testLinksSelect)
	t.Run("OrganizationMembers", testOrganizationMembersSelect)
	t.Run("Organizations", testOrganizationsSelect)
	t.Run("Repositories", testRepositoriesSelect)
	t.Run("SchemaMigrations", testSchemaMigrationsSelect)
	t.Run("Sessions", testSessionsSelect)
	t.Run("Synonyms", testSynonymsSelect)
	t.Run("Topics", testTopicsSelect)
	t.Run("UserLinkTopics", testUserLinkTopicsSelect)
	t.Run("UserLinks", testUserLinksSelect)
	t.Run("Users", testUsersSelect)
}

func TestUpdate(t *testing.T) {
	t.Run("Links", testLinksUpdate)
	t.Run("OrganizationMembers", testOrganizationMembersUpdate)
	t.Run("Organizations", testOrganizationsUpdate)
	t.Run("Repositories", testRepositoriesUpdate)
	t.Run("SchemaMigrations", testSchemaMigrationsUpdate)
	t.Run("Sessions", testSessionsUpdate)
	t.Run("Synonyms", testSynonymsUpdate)
	t.Run("Topics", testTopicsUpdate)
	t.Run("UserLinkTopics", testUserLinkTopicsUpdate)
	t.Run("UserLinks", testUserLinksUpdate)
	t.Run("Users", testUsersUpdate)
}

func TestSliceUpdateAll(t *testing.T) {
	t.Run("Links", testLinksSliceUpdateAll)
	t.Run("OrganizationMembers", testOrganizationMembersSliceUpdateAll)
	t.Run("Organizations", testOrganizationsSliceUpdateAll)
	t.Run("Repositories", testRepositoriesSliceUpdateAll)
	t.Run("SchemaMigrations", testSchemaMigrationsSliceUpdateAll)
	t.Run("Sessions", testSessionsSliceUpdateAll)
	t.Run("Synonyms", testSynonymsSliceUpdateAll)
	t.Run("Topics", testTopicsSliceUpdateAll)
	t.Run("UserLinkTopics", testUserLinkTopicsSliceUpdateAll)
	t.Run("UserLinks", testUserLinksSliceUpdateAll)
	t.Run("Users", testUsersSliceUpdateAll)
}
