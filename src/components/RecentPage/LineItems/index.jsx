// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LineItems_view as View } from './__generated__/LineItems_view.graphql'
import LineItem from './LineItem'

type Props = {|
  +view: View,
|}

const Placeholder = () => <div>No recent activity</div>

const LineItems = ({ view }: Props) => {
  const edges = view ? view.activity.edges : null

  if (!edges)
    return <Placeholder />

  return (
    <div className="px-3 py-4 px-md-6 px-lg-0">
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          Recent activity
        </div>
      </div>
      <div className="Box">
        { edges.map(e => e && e.node && (
          <LineItem key={e.node.createdAt} item={e.node} />
        )) }
      </div>
    </div>
  )
}

export default createFragmentContainer(LineItems, graphql`
  fragment LineItems_view on View {
    activity(first: 50) {
      edges {
        node {
          createdAt
          description
        }
      }
    }
  }
`)
