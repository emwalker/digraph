import React from 'react'
import { graphql, useLazyLoadQuery } from 'react-relay'
import { Location } from 'found'

import TopicSearchPage from './TopicSearchPage'
import ViewTopicPage from './ViewTopicPage'
import {
  TopicPage_query_Query,
  TopicPage_query_Query$variables,
} from '__generated__/TopicPage_query_Query.graphql'

type Props = {
  location: Location,
  variables: TopicPage_query_Query$variables,
}

export const query = graphql`
  query TopicPage_query_Query(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: String!,
    $searchString: String!,
  ) {
    alerts {
      id
      text
      type
    }

    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...ViewTopicPage_viewer
        ...TopicSearchPage_viewer
      }

      topic(id: $topicId) {
        ...ViewTopicPage_topic @arguments(searchString: $searchString)
        ...TopicSearchPage_topic @arguments(searchString: $searchString)
      }
    }
  }
`

export default function TopicPage({ location, variables }: Props) {
  const data = useLazyLoadQuery<TopicPage_query_Query>(query, variables)

  const { topic, viewer } = data.view
  if (!topic || !viewer) return null

  if (location.query.q) {
    return (
      <TopicSearchPage
        topic={topic}
        viewer={viewer}
      />
    )
  }

  return (
    <ViewTopicPage
      location={location}
      topic={topic}
      viewer={viewer}
    />
  )
}
