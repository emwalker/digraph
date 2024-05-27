/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    " query SearchBox(\n  $repoIds: [ID!]!, $searchString: String!, $topicSynonymSearchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {\n    topicLiveSearch(searchString: $topicSynonymSearchString) {\n      synonyms {\n        displayName\n        id\n      }\n    }\n\n    queryInfo {\n      topics {\n        displayName\n        id\n      }\n      phrases\n    }\n  }\n}": types.SearchBoxDocument,
    " query SearchResults(\n  $repoIds: [ID!]!, $topicId: ID!, $searchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {  \n    topic(id: $topicId) {\n      displayName\n      displaySynonyms {\n        name\n      }\n\n      displayParentTopics(first: 10) {\n        edges {\n          node {\n            id\n            displayName\n          }\n        }\n      }\n\n      children(searchString: $searchString, first: 50) {\n        edges {\n          node {\n            ... on Topic {\n              id\n              displayName\n            }\n\n            ... on Link {\n              id\n              displayTitle\n              displayUrl\n            }\n          }\n        }\n      }\n    }\n  }\n}": types.SearchResultsDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: " query SearchBox(\n  $repoIds: [ID!]!, $searchString: String!, $topicSynonymSearchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {\n    topicLiveSearch(searchString: $topicSynonymSearchString) {\n      synonyms {\n        displayName\n        id\n      }\n    }\n\n    queryInfo {\n      topics {\n        displayName\n        id\n      }\n      phrases\n    }\n  }\n}"): (typeof documents)[" query SearchBox(\n  $repoIds: [ID!]!, $searchString: String!, $topicSynonymSearchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {\n    topicLiveSearch(searchString: $topicSynonymSearchString) {\n      synonyms {\n        displayName\n        id\n      }\n    }\n\n    queryInfo {\n      topics {\n        displayName\n        id\n      }\n      phrases\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: " query SearchResults(\n  $repoIds: [ID!]!, $topicId: ID!, $searchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {  \n    topic(id: $topicId) {\n      displayName\n      displaySynonyms {\n        name\n      }\n\n      displayParentTopics(first: 10) {\n        edges {\n          node {\n            id\n            displayName\n          }\n        }\n      }\n\n      children(searchString: $searchString, first: 50) {\n        edges {\n          node {\n            ... on Topic {\n              id\n              displayName\n            }\n\n            ... on Link {\n              id\n              displayTitle\n              displayUrl\n            }\n          }\n        }\n      }\n    }\n  }\n}"): (typeof documents)[" query SearchResults(\n  $repoIds: [ID!]!, $topicId: ID!, $searchString: String!, $viewerId: ID!\n) {\n  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {  \n    topic(id: $topicId) {\n      displayName\n      displaySynonyms {\n        name\n      }\n\n      displayParentTopics(first: 10) {\n        edges {\n          node {\n            id\n            displayName\n          }\n        }\n      }\n\n      children(searchString: $searchString, first: 50) {\n        edges {\n          node {\n            ... on Topic {\n              id\n              displayName\n            }\n\n            ... on Link {\n              id\n              displayTitle\n              displayUrl\n            }\n          }\n        }\n      }\n    }\n  }\n}"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;