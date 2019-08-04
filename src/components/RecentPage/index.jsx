// @flow
import React from 'react'
import { graphql } from 'react-relay'
import DocumentTitle from 'react-document-title'

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
  <Container>
    <div className="blankslate">
      <p>Searching the servers for recent activity ...</p>
    </div>
  </Container>
)

export default ({ props }: WrapperProps) => (
  <DocumentTitle title="Recent activity | Digraph">
    {
      // eslint-disable-next-line react/prop-types
      props && props.view && props.view.topic
        // eslint-disable-next-line react/prop-types
        ? <LineItems topic={props.view.topic} />
        : <Placeholder />
    }
  </DocumentTitle>
)

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
