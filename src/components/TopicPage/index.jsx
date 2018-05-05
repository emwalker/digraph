import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import ListView from '../ui/ListView'
import { liftNodes } from '../../utils'

type Props = {
  topic: {
    name: string,
  }
}

const TopicPage = ({ topic, ...props }: Props) => {
  const { name, links } = topic
  return (
    <ListView
      title={name}
      items={liftNodes(links)}
      {...props}
    >
      Sidebar TDB
    </ListView>
  )
}

export const query = graphql`
query TopicPage_query_Query(
  $organizationId: String!,
  $topicId: String!
) {
  viewer {
    ...TopicPage_viewer
  }

  organization(resourceId: $organizationId) {
    ...TopicPage_organization

    topic(resourceId: $topicId) {
      ...TopicPage_topic
    }
  }
}`

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_viewer on User {
    ...LinkList_viewer
  }

  fragment TopicPage_organization on Organization {
    ...LinkList_organization
  }

  fragment TopicPage_topic on Topic {
    name

    links(first: 100) {
      edges {
        node {
          id
          display: title
          resourcePath: url
        }
      }
    }
  }
`)
