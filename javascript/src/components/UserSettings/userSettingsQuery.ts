import { graphql } from 'react-relay'

import {
  userSettingsQuery_query_QueryResponse as Response,
} from '__generated__/userSettingsQuery_query_Query.graphql'

export type ViewType = Response['view']

export default graphql`
  query userSettingsQuery_query_Query(
    $viewerId: ID!,
    $repoIds: [ID!],
  ) {
    alerts {
      id
      text
      type
    }

    view(
      viewerId: $viewerId,
      repositoryIds: $repoIds,
    ) {
      ...UserSettings_view
    }
  }
`
