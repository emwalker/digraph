import React from 'react'
import { graphql, useFragment } from 'react-relay'
import { isEmpty } from 'ramda'

import Page from 'components/ui/Page'
import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import { liftNodes, NodeTypeOf } from 'components/types'
import {
  TopicSearchPage_viewer$key,
  TopicSearchPage_viewer$data as ViewerType,
} from '__generated__/TopicSearchPage_viewer.graphql'
import {
  TopicSearchPage_topic$key,
  TopicSearchPage_topic$data as TopicType,
} from '__generated__/TopicSearchPage_topic.graphql'

type ParentTopicType = NodeTypeOf<TopicType['displayParentTopics']>
type SearchItemType = NodeTypeOf<TopicType['children']>

type Props = {
  topic: TopicSearchPage_topic$key,
  viewer: TopicSearchPage_viewer$key,
}

const topicFragment = graphql`
  fragment TopicSearchPage_topic on Topic @argumentDefinitions(
    searchString: {type: "String"},
  ) {
    id
    displayName

    displayParentTopics(first: 100) {
      edges {
        node {
          displayName
          id
        }
      }
    }

    children(first: 100, searchString: $searchString) {
      edges {
        node {
          __typename

          ... on Topic {
            id
            ...Topic_topic
          }

          ... on Link {
            id
            ...Link_link
          }
        }
      }
    }
  }
`

const viewerFragment = graphql`
  fragment TopicSearchPage_viewer on User {
    id
    ...Topic_viewer
    ...Link_viewer
  }
`

const renderSearchResultItem = (viewer: ViewerType, item: SearchItemType | null) => {
  if (!item) return null

  if (item.__typename === 'Topic') {
    return (
      <Topic
        key={item.id}
        topic={item}
        viewer={viewer}
      />
    )
  }

  if (item.__typename === 'Link') {
    return (
      <Link
        key={item.id}
        link={item}
        viewer={viewer}
      />
    )
  }

  return null
}

export default function TopicSearchPage(props: Props) {
  const topic = useFragment(topicFragment, props.topic)
  const viewer = useFragment(viewerFragment, props.viewer)

  if (topic == null) return <div>Error parsing route</div>

  const {
    children: searchResults,
    displayName,
    displayParentTopics,
  } = topic
  const items = liftNodes<SearchItemType>(searchResults)

  return (
    <Page>
      <div className="px-3 px-md-6 px-lg-0">
        <Subhead
          heading={displayName}
        />
        <Columns>
          <RightColumn>
            <SidebarList
              items={liftNodes<ParentTopicType>(displayParentTopics)}
              placeholder="There are no parent topics for this topic."
              title="Parent topics"
            />
          </RightColumn>
          <LeftColumn>
            <List
              placeholder="There are no items in this list."
              hasItems={!isEmpty(items)}
            >
              {items.map((item) => renderSearchResultItem(viewer, item))}
            </List>
          </LeftColumn>
        </Columns>
      </div>
    </Page>
  )
}
