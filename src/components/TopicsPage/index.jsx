// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import ItemList from '../ui/ItemList'
import { liftNodes } from '../../utils'

type Props = {
  organization: {
    topics: Object,
  },
}

const TopicsPage = ({ organization: { topics } }: Props) => (
  <div>
    <div className="Subhead">
      <div className="Subhead-heading">Topics</div>
    </div>
    <ItemList
      title="Topics"
      items={liftNodes(topics)}
    />
  </div>
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

    topics(first: 1000) @connection(key: "Organization_topics") {
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
