import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import { LineItems_activity as Activity } from '__generated__/LineItems_activity.graphql'
import LineItem from './LineItem'
import Container from '../Container'

type Props = {
  activity: Activity,
}

const NoRecentActivity = () => (
  <Container>
    <div className="blankslate">
      <p>No recent activity.</p>
    </div>
  </Container>
)

const LineItems = ({ activity }: Props) => {
  const edges = activity?.edges

  if (!edges || edges.length === 0) return <NoRecentActivity />

  return (
    <Container>
      { edges.map((e) => e && e.node && (
        <LineItem key={e.node.createdAt as string} item={e.node} />
      )) }
    </Container>
  )
}

export default createFragmentContainer(LineItems, {
  activity: graphql`
    fragment LineItems_activity on ActivityLineItemConnection {
      edges {
        node {
          createdAt
          description
        }
      }
    }
  `,
})
