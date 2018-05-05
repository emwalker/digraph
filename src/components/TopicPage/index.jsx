import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import ListView from '../ui/ListView'
import SidebarList from '../ui/SidebarList'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import { liftNodes } from '../../utils'

type Props = {
  topic: {
    name: string,
  }
}

const TopicPage = ({ topic, ...props }: Props) => {
  const {
    childTopics,
    links,
    name,
    parentTopics,
  } = topic
  const items = liftNodes(childTopics).concat(liftNodes(links))

  return (
    <ListView
      title={name}
      items={items}
      {...props}
    >
      <SidebarList
        title="Parent topics"
        items={liftNodes(parentTopics)}
      />
      <AddTopic
        topic={topic}
        {...props}
      />
      <AddLink
        topic={topic}
        {...props}
      />
    </ListView>
  )
}

export const query = graphql`
query TopicPage_query_Query(
  $organizationId: String!,
  $topicId: String!
) {
  organization(resourceId: $organizationId) {
    ...TopicPage_organization

    topic(resourceId: $topicId) {
      ...TopicPage_topic
    }
  }
}`

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_organization on Organization {
    ...AddTopic_organization
    ...AddLink_organization
  }

  fragment TopicPage_topic on Topic {
    name
    ...AddTopic_topic
    ...AddLink_topic

    parentTopics(first: 100) {
      edges {
        node {
          display: name
          resourcePath
        }
      }
    }

    childTopics(first: 100) @connection(key: "Topic_childTopics") {
      edges {
        node {
          __typename
          display: name
          resourcePath
        }
      }
    }

    links(first: 100)  @connection(key: "Topic_links") {
      edges {
        node {
          __typename
          display: title
          resourcePath: url
        }
      }
    }
  }
`)
