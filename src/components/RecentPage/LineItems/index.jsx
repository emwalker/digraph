// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LineItems_topic as Topic } from './__generated__/LineItems_topic.graphql'
import LineItem from './LineItem'
import Container from '../Container'

type Props = {|
  +topic: Topic,
|}

const NoRecentActivity = () => (
  <Container>
    <div className="blankslate">
      <p>No recent activity.</p>
    </div>
  </Container>
)

const LineItems = ({ topic }: Props) => {
  const edges = topic && topic.activity ? topic.activity.edges : null

  if (!edges) return <NoRecentActivity topic={topic} />

  return (
    <Container topic={topic}>
      { edges.map(e => e && e.node && (
        <LineItem key={e.node.createdAt} item={e.node} />
      )) }
    </Container>
  )
}

export default createFragmentContainer(LineItems, {
  topic: graphql`
    fragment LineItems_topic on Topic {
      ...Container_topic

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
