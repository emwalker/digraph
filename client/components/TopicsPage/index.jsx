// @flow
import React from 'react'
import { compose, isNil, map, prop, propOr, reject } from 'ramda'
import { graphql, createFragmentContainer } from 'react-relay'
import { ListGroup, ListGroupItem } from 'reactstrap'

import AddTopic from './AddTopic'

const topicList = compose(reject(isNil), map(prop('node')), propOr([], 'edges'))

type Props = {
  organization: {
    topics: Object,
  },
  relay: {
    environment: Object,
  }
}

const TopicsPage = ({ organization, relay }: Props) => (
  <div>
    <h1>Topics</h1>
    <div className="row">
      <div className="col">
        <ListGroup>
          {topicList(organization.topics).map(({ id, name, resourcePath }) => (
            <ListGroupItem key={id} tag="a" href={resourcePath}>{name}</ListGroupItem>
          ))}
        </ListGroup>
      </div>
      <div className="col-5">
        <AddTopic
          className="test-add-topic"
          organization={organization}
          relay={relay}
        />
      </div>
    </div>
  </div>
)

export default createFragmentContainer(TopicsPage, graphql`
  fragment TopicsPage_viewer on User {
    name
  }

  fragment TopicsPage_organization on Organization {
    id
    resourceId

    topics(first: 100) @connection(key: "Organization_topics") {
      edges {
        node {
          id
          name
          resourcePath
          description
        }
      }
    }
  }
`)
