// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import List from '../ui/List'
import Topic from '../ui/Topic'
import { liftNodes } from '../../utils'

type Props = {
  organization: {
    topics: Object,
  },
}

const TopicsPage = ({ organization, ...props }: Props) => {
  const topics = liftNodes(organization.topics)
  return (
    <div>
      <div className="Subhead">
        <div className="Subhead-heading">Topics</div>
      </div>
      <List
        placeholder="There are no topics"
        hasItems={!isEmpty(topics)}
      >
        { topics.map(topic => (
          <Topic
            key={topic.resourcePath}
            topic={topic}
            organization={organization}
            {...props}
          />
        )) }
      </List>
    </div>
  )
}

export const query = graphql`
query TopicsPage_query_Query($organizationId: String!) {
  organization(resourceId: $organizationId) {
    ...TopicsPage_organization
  }
}`

export default createFragmentContainer(TopicsPage, graphql`
  fragment TopicsPage_organization on Organization {
    id
    resourceId
    ...Topic_organization

    topics(first: 1000) @connection(key: "Organization_topics") {
      edges {
        node {
          resourcePath
          ...Topic_topic
        }
      }
    }
  }
`)
