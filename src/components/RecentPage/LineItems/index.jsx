// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LineItems_view as View } from './__generated__/LineItems_view.graphql'
import LineItem from './LineItem'
import Container from '../Container'

type Props = {|
  +view: View,
|}

const NoRecentActivity = () => (
  <Container>
    <div className="blankslate">
      <p>No recent activity.</p>
    </div>
  </Container>
)

const LineItems = ({ view }: Props) => {
  const edges = view ? view.activity.edges : null

  if (!edges) return <NoRecentActivity />

  return (
    <Container>
      { edges.map(e => e && e.node && (
        <LineItem key={e.node.createdAt} item={e.node} />
      )) }
    </Container>
  )
}

export default createFragmentContainer(LineItems, {
  view: graphql`
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
  `,
})
