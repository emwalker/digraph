import { graphql } from 'react-relay'

import {
  userSettingsQuery_query_QueryResponse as Response,
} from '__generated__/userSettingsQuery_query_Query.graphql'

export type ViewType = Response['view']

export default graphql`
  query userSettingsQuery_query_Query(
    $viewerId: ID!,
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
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
      ...UserSettings_view
    }
  }
`
