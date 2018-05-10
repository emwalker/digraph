// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import type { TopicType } from '../types'
import SidebarList from '../ui/SidebarList'
import List from '../ui/List'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import { liftNodes } from '../../utils'
import Link from '../ui/Link'
import Topic from '../ui/Topic'

type Props = {
  topic: TopicType,
}

const TopicPage = ({ topic, ...props }: Props) => {
  const {
    childTopics,
    links: childLinks,
    name,
    parentTopics,
  } = topic
  const topics = liftNodes(childTopics)
  const links = liftNodes(childLinks)

  return (
    <div>
      <div className="Subhead">
        <div className="Subhead-heading">{name}</div>
      </div>
      <div className="one-third column pl-0">
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
      </div>
      <div className="two-thirds column pr-0">
        <List
          placeholder="There are no items in this list."
          hasItems={!isEmpty(topics) || !isEmpty(links)}
        >
          { topics.map(childTopic => (
            <Topic
              key={childTopic.resourceId}
              topic={childTopic}
              {...props}
            />
          )) }

          { links.map(link => (
            <Link
              key={link.resourceId}
              link={link}
              {...props}
            />
          )) }
        </List>
      </div>
    </div>
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
    ...Link_organization
    ...Topic_organization
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

    childTopics(first: 1000) @connection(key: "Topic_childTopics") {
      edges {
        node {
          resourceId
          ...Topic_topic
        }
      }
    }

    links(first: 1000)  @connection(key: "Topic_links") {
      edges {
        node {
          resourceId
          ...Link_link
        }
      }
    }
  }
`)
