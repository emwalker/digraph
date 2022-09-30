import { graphql } from 'react-relay'

import {
  reviewPageQuery_query_Query$data as Response,
} from '__generated__/reviewPageQuery_query_Query.graphql'

export type ViewType = Response['view']

export default graphql`
  query reviewPageQuery_query_Query(
    $repoIds: [ID!],
    $topicId: ID!,
    $viewerId: ID!,
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
      ...ReviewPage_view
    }
  }
`
