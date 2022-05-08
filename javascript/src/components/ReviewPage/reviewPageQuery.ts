import { graphql } from 'react-relay'

import {
  reviewPageQuery_query_QueryResponse as Response,
} from '__generated__/reviewPageQuery_query_Query.graphql'

export type ViewType = Response['view']

export default graphql`
query reviewPageQuery_query_Query(
  $viewerId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $topicId: ID!,
) {
  alerts {
    id
    text
    type
  }

  view(
    viewerId: $viewerId,
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    ...ReviewPage_view
  }
}`
