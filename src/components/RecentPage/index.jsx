// @flow
import React from 'react'
import { graphql } from 'react-relay'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import LineItems from './LineItems'
import type { RecentPage_recent_QueryResponse as Response } from './__generated__/RecentPage_recent_Query.graphql'
import Container from './Container'

type View = $NonMaybeType<$PropertyType<Response, 'view'>>

type Props = {|
  // $FlowFixMe
  +view: View,
|}

type WrapperProps = {|
  props: ?Props,
|}

const Placeholder = () => (
  <Container topicName={null}>
    <div className="blankslate">
      <p>Searching the servers for recent activity ...</p>
    </div>
  </Container>
)

export default ({ props }: WrapperProps) => {
  useDocumentTitle('Recent activity | Digraph')

  return (
    <Page>
      {
        // eslint-disable-next-line react/prop-types
        props && props.view && props.view.topic
          // eslint-disable-next-line react/prop-types
          ? <LineItems topic={props.view.topic} />
          : <Placeholder />
      }
    </Page>
  )
}

export const query = graphql`
query RecentPage_recent_Query(
  $viewerId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $topicId: ID!,
) {
  view(
    viewerId: $viewerId,
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    topic(id: $topicId) {
      ...LineItems_topic
    }
  }
}`
