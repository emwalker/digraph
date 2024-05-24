import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { LineItems_activity$key } from '__generated__/LineItems_activity.graphql'
import LineItem from './LineItem'
import Container from '../Container'

type Props = {
  activity: LineItems_activity$key,
}

const NoRecentActivity = () => (
  <Container>
    <div className="blankslate">
      <p>No recent activity.</p>
    </div>
  </Container>
)

export default function LineItems(props: Props) {
  const activity = useFragment(
    graphql`
      fragment LineItems_activity on ActivityLineItemConnection {
        edges {
          node {
            createdAt
            description
          }
        }
      }
    `,
    props.activity,
  )
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
