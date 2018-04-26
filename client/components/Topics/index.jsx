// @flow
import React from 'react'
import { compose, isNil, map, prop, propOr, reject } from 'ramda'
import { graphql, createFragmentContainer } from 'react-relay'

import Topic from './Topic'
import AddTopic from './AddTopic'

const topicList = compose(reject(isNil), map(prop('node')), propOr([], 'edges'))

type Props = {
  viewer: {
    name: string,
  },
  organization: {
    topics: Object,
  },
  relay: {
    environment: Object,
  }
}

const Topics = ({ viewer: { name }, organization, relay }: Props) => (
  <div>
    <h1>Topics</h1>
    <div className="row">
      <div className="col">
        <p className="lead">
          List of topics visible to { name }
        </p>
      </div>
      <div className="col-6">
        <AddTopic
          className="test-add-topic"
          organization={organization}
          relay={relay}
        />
      </div>
    </div>
    <ul>
      {topicList(organization.topics).map(topic => (
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
    id
    resourceId

    topics(first: 100) @connection(key: "Organization_topics") {
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
