// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import AddTopic from './AddTopic'
import ListView from '../ui/ListView'
import { liftNodes } from '../../utils'

type Props = {
  organization: {
    topics: Object,
  },
  relay: {
    environment: Object,
  }
}

const TopicsPage = ({ organization, relay }: Props) => (
  <ListView
    title="Topics"
    items={liftNodes(organization.topics)}
  >
    <AddTopic
      className="test-add-topic"
      organization={organization}
      relay={relay}
    />
  </ListView>
)

export const query = graphql`
query TopicsPage_query_Query($organizationId: String!) {
  viewer {
    ...TopicsPage_viewer
  }

  organization(resourceId: $organizationId) {
    ...TopicsPage_organization
  }
}`

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
          display: name
          resourcePath
        }
      }
    }
  }
`)
