// @flow
import React from 'react'
import { graphql } from 'react-relay'

import LineItems from './LineItems'
import type { RecentPage_recent_QueryResponse as Response } from './__generated__/RecentPage_recent_Query.graphql'

type View = $NonMaybeType<$PropertyType<Response, 'view'>>

type Props = {|
  props: {
    +view: View,
  },
|}

const Placeholder = () => <div>No recent activity</div>

export default ({ props }: Props) => {
  // eslint-disable-next-line react/prop-types
  if (!props || !props.view)
    return <Placeholder />

  // $FlowFixMe
  return <LineItems view={props.view} />
}

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
