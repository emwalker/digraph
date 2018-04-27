// @flow
import React from 'react'
import { compose, map, prop, propOr } from 'ramda'
import { graphql, createFragmentContainer } from 'react-relay'

import Topic from './Topic'

const topicList = compose(map(prop('node')), propOr([], 'edges'))

type Props = {
  viewer: {
    name: string,
  },
  organization: {
    topics: Object,
  }
}

const Topics = ({ viewer: { name }, organization: { topics } }: Props) => (
  <div>
    <h1>Topics</h1>
    <p className="lead">
      List of topics visible to { name }
    </p>
    <ul>
      {topicList(topics).map(topic => (
        <Topic key={topic.id} topic={topic} />
      ))}
    </ul>
  </div>
)

export default createFragmentContainer(Topics, graphql`
  fragment Topics_viewer on User {
    name
  }

  fragment Topics_organization on Organization {
    topics(first: 100) {
      edges {
        node {
          id
          name
          resourceId
          description
        }
      }
    }
  }
`)
