import React from 'react'
import { graphql } from 'react-relay'
import { Match } from 'found'

import TopicSearchPage from './TopicSearchPage'
import ViewTopicPage from './ViewTopicPage'
import { TopicPage_query_Query$data } from '__generated__/TopicPage_query_Query.graphql'

export const query = graphql`
  query TopicPage_query_Query(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: ID!,
    $searchString: String,
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

type ViewType = TopicPage_query_Query$data['view']

type TopicPageProps = {
  view: ViewType,
  match: Match,
}

export function TopicPage({ match: { location }, view }: TopicPageProps) {
  if (!view || !view.topic || !view.viewer) return <div>Loading ...</div>

  if (location.query.q) {
    return (
      <TopicSearchPage
        topic={view.topic}
        viewer={view.viewer}
      />
    )
  }

  return (
    <ViewTopicPage
      location={location}
      topic={view.topic}
      viewer={view.viewer}
    />
  )
}
