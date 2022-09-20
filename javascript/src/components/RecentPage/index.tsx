import React from 'react'
import { graphql } from 'react-relay'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import {
  RecentPage_recent_Query$data as Response,
} from '__generated__/RecentPage_recent_Query.graphql'
import LineItems from './LineItems'
import Container from './Container'

type ViewType = Response['view']

type Props = {
  view: ViewType,
}

const Placeholder = () => (
  <Container>
    <div className="blankslate">
      <p>Searching the servers for recent activity ...</p>
    </div>
  </Container>
)

export default ({ view }: Props) => {
  useDocumentTitle('Recent activity | Digraph')

  return (
    <Page>
      {
        view
          ? <LineItems activity={view.activity} />
          : <Placeholder />
      }
    </Page>
  )
}

export const query = graphql`
query RecentPage_recent_Query(
  $viewerId: ID!,
  $repoIds: [ID!],
  $topicId: String,
) {
  view(
    repoIds: $repoIds,
    viewerId: $viewerId,
  ) {
    activity(topicId: $topicId, first: 50) {
      ...LineItems_activity
    }
  }
}`
