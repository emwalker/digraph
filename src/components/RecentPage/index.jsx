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
    { 'No recent activity' }
  </Container>
)

const Recents = ({ view }: Props) => (
  <DocumentTitle title="Recent activity | Digraph">
    <LineItems view={view} />
  </DocumentTitle>
)

export default ({ props }: WrapperProps) => (
  // eslint-disable-next-line react/prop-types
  props && props.view
    // eslint-disable-next-line react/prop-types
    ? <Recents view={props.view} />
    : <Placeholder />
)

export const query = graphql`
query RecentPage_recent_Query(
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
) {
  view(
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    ...LineItems_view
  }
}`
