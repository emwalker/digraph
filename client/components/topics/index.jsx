// @flow
import React from 'react'
import { compose, map, prop, propOr } from 'ramda'

import Layout from '../Layout'
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

export default ({ viewer: { name }, organization: { topics } }: Props) => (
  <Layout>
    <h1>Topics</h1>
    <p className="lead">
      List of topics visible to { name }
    </p>
    <ul>
      {topicList(topics).map(topic => (
        <Topic key={topic.id} topic={topic} />
      ))}
    </ul>
  </Layout>
)
