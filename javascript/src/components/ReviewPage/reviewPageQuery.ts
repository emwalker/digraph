import { graphql } from 'react-relay'

import {
  reviewPageQuery_query_QueryResponse as Response,
} from '__generated__/reviewPageQuery_query_Query.graphql'

export type ViewType = Response['view']

export default graphql`
query reviewPageQuery_query_Query(
  $viewerId: ID!,
  $repoIds: [ID!],
  $topicId: String!,
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
    ...ReviewPage_view
  }
}`
