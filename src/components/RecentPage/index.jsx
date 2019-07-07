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
      props && props.view
        // eslint-disable-next-line react/prop-types
        ? <LineItems view={props.view} />
        : <Placeholder />
    }
  </DocumentTitle>
)

export const query = graphql`
query RecentPage_recent_Query(
  $viewerId: ID!,
  $sessionId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
) {
  view(
    viewerId: $viewerId,
    sessionId: $sessionId,
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    ...LineItems_view
  }
}`
