// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import List from 'components/ui/List'
import type { TopicType } from '../types'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import { liftNodes } from '../../utils'
import Link from '../ui/Link'
import Topic from '../ui/Topic'

type Props = {
  location: Object,
  orgLogin: string,
  router: Object,
  topic: TopicType,
  viewer: {
    id: string,
  },
}

const TopicPage = ({ location, router, topic, viewer, ...props }: Props) => {
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
      <Subhead
        heading={name}
        location={location}
        router={router}
      />
      <div className="two-thirds column pl-0">
        <List
          placeholder="There are no items in this list."
          hasItems={!isEmpty(topics) || !isEmpty(links)}
        >
          { topics.map(childTopic => (
            <Topic
              key={childTopic.id}
              topic={childTopic}
              {...props}
            />
          )) }

          { links.map(link => (
            <Link
              key={link.id}
              link={link}
              {...props}
            />
          )) }
        </List>
      </div>
      <div className="one-third column pr-0">
        <SidebarList
          title="Parent topics"
          items={liftNodes(parentTopics)}
        />
        <AddTopic
          topic={topic}
          viewer={viewer}
          {...props}
        />
        <AddLink
          topic={topic}
          viewer={viewer}
          {...props}
        />
      </div>
    </div>
  )
}

export const query = graphql`
query TopicPage_query_Query(
  $repoIds: [ID!],
  $topicId: ID!,
  $searchString: String,
) {
  viewer {
    id
  }

  view(repositoryIds: $repoIds) {
    topic(id: $topicId) {
      ...TopicPage_topic @arguments(searchString: $searchString)
    }
  }
}`

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_topic on Topic @argumentDefinitions(
    searchString: {type: "String", defaultValue: ""},
  ) {
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

    childTopics(first: 1000, searchString: $searchString) @connection(key: "Topic_childTopics") {
      edges {
        node {
          id
          ...Topic_topic
        }
      }
    }

    links(first: 1000, searchString: $searchString)  @connection(key: "Topic_links") {
      edges {
        node {
          id
          ...Link_link
        }
      }
    }
  }
`)
