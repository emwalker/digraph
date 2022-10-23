import React, { useCallback } from 'react'
import { graphql, useFragment } from 'react-relay'
import { isEmpty } from 'ramda'
import { Link as FoundLink, Location } from 'found'

import Page from 'components/ui/Page'
import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import RepoOwnership from 'components/ui/RepoOwnership'
import { topicPath } from 'components/helpers'
import { Color, NodeTypeOf, liftNodes } from 'components/types'
import {
  ViewTopicPage_viewer$key,
  ViewTopicPage_viewer$data as ViewerType,
} from '__generated__/ViewTopicPage_viewer.graphql'
import {
  ViewTopicPage_topic$key,
  ViewTopicPage_topic$data as TopicType,
} from '__generated__/ViewTopicPage_topic.graphql'
import AddForm from './AddForm'

type TopicChildType = NodeTypeOf<TopicType['children']>

type Props = {
  location: Location,
  topic: ViewTopicPage_topic$key,
  viewer: ViewTopicPage_viewer$key,
}

const viewerFragment = graphql`
  fragment ViewTopicPage_viewer on User {
    id
    ...AddForm_viewer
  }
`

const topicFragment = graphql`
  fragment ViewTopicPage_topic on Topic
  @argumentDefinitions(searchString: {type: "String!", defaultValue: ""})
  {
    displayName
    id
    showRepoOwnership
    viewerCanUpdate

    repoTopics {
      displayColor
    }

    displaySynonyms {
      name
    }

    displayParentTopics(first: 100) {
      edges {
        node {
          displayName
          id
        }
      }
    }

    children(first: 1000, searchString: $searchString)
    @connection(key: "ViewTopicPage_topic_children") {
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

    ...AddForm_topic
  }
`

type TopicChildProps = {
  child: TopicChildType | null,
  viewer: ViewerType,
}

function TopicChild({ child, viewer }: TopicChildProps) {
  if (!child) return null

  if (child.__typename == 'Topic') {
    return (
      <Topic
        topic={child}
        viewerId={viewer.id}
      />
    )
  }

  if (child.__typename == 'Link') {
    return (
      <Link
        link={child}
        viewerId={viewer.id}
      />
    )
  }

  return null
}

function renderRepoOwnership(topic: TopicType) {
  const repoColors = topic.repoTopics.map((repoTopic) => repoTopic.displayColor as Color)

  return (
    <div className="mt-2">
      <RepoOwnership
        showRepoOwnership={topic.showRepoOwnership}
        repoColors={repoColors}
      />
    </div>
  )
}

function headingDetail(topic: TopicType) {
  const length = topic.displaySynonyms.length

  if (length < 2) return <div>{renderRepoOwnership(topic)}</div>

  return (
    <div>
      <div className="displaySynonyms h6">
        {topic.displaySynonyms.map(({ name }) => (
          <span key={name} className="synonym">{name}</span>
        ))}
      </div>

      {renderRepoOwnership(topic)}
    </div>
  )
}

const renderNotification = () => (
  <div className="Box p-3 mt-3">
    You must be
    {' '}
    <a href="/login">signed in</a>
    {' '}
    to add and move topics and links.
  </div>
)

function renderTopicViews(topic: TopicType) {
  const recentActivityLocation = {
    pathname: `${topicPath(topic.id)}/recent`,
    query: {},
    search: '',
    state: {
      itemTitle: topic.displayName,
    },
  }

  return (
    <div className="Box Box--condensed mb-3">
      <div className="Box-header">
        <span className="Box-title">This topic</span>
      </div>
      <ul>
        <li className="Box-row">
          <FoundLink to={recentActivityLocation} className="Box-row-link">
            Recent activity
          </FoundLink>
        </li>
      </ul>
    </div>
  )
}

export default function TopicPage(props: Props) {
  const topic = useFragment(topicFragment, props.topic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const children = liftNodes(topic.children)
  const parentTopics = liftNodes(topic.displayParentTopics)
  const renderHeadingDetail = useCallback(() => headingDetail(topic), [headingDetail, topic])

  const canEdit = topic.viewerCanUpdate

  return (
    <Page>
      <Subhead
        heading={topic.displayName}
        renderHeadingDetail={renderHeadingDetail}
      />
      <Columns>
        <RightColumn>
          <SidebarList
            items={parentTopics}
            placeholder="There are no parent topics for this topic."
            title="Parent topics"
          />
          {renderTopicViews(topic)}
          {canEdit
            ? <AddForm topic={topic} viewer={viewer} />
            : renderNotification()}
        </RightColumn>
        <LeftColumn>
          <List
            placeholder="There are no items in this list."
            hasItems={!isEmpty(children)}
          >
            {children.filter(Boolean).map((child) => {
              const key = (child?.__typename == 'Link' || child?.__typename == 'Topic')
                ? child.id
                : 'client:topicChildren:0'

              return (
                <TopicChild
                  key={key}
                  child={child}
                  viewer={viewer}
                />
              )
            })}
          </List>
        </LeftColumn>
      </Columns>
    </Page>
  )
}
