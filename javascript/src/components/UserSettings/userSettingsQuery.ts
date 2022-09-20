import { graphql } from 'react-relay'

import {
  userSettingsQuery_query_Query$data as Response,
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
      repoIds: $repoIds,
      viewerId: $viewerId,
    ) {
      ...UserSettings_view
    }
  }
`
