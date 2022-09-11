import React, { useCallback } from 'react'
import { graphql, useFragment } from 'react-relay'
import { isEmpty } from 'ramda'
import { Link as FoundLink } from 'found'

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
import { Color, LocationType, NodeTypeOf, liftNodes } from 'components/types'
import { TopicPage_query_Query$data as Response } from '__generated__/TopicPage_query_Query.graphql'
import {
  TopicPage_topic$key,
  TopicPage_topic$data as TopicType,
} from '__generated__/TopicPage_topic.graphql'
import AddForm from './AddForm'

type ViewType = Response['view']
type TopicChildType = NodeTypeOf<TopicType['children']>
type ParentTopicType = NodeTypeOf<TopicType['displayParentTopics']>

type Props = {
  // eslint-disable-next-line react/no-unused-prop-types
  alerts: Object[],
  location: LocationType,
  orgLogin: string,
  topic: TopicPage_topic$key,
  view: ViewType,
}

export const query = graphql`
query TopicPage_query_Query(
  $viewerId: ID!,
  $repoIds: [ID!],
  $topicId: String!,
  $searchString: String,
) {
  alerts {
    id
    text
    type
  }

  view(
    viewerId: $viewerId,
    repositoryIds: $repoIds,
  ) {
    viewer {
      id
      isGuest
      ...AddForm_viewer
      ...Link_viewer
    }

    topic(id: $topicId) {
      ...TopicPage_topic @arguments(searchString: $searchString)
    }
  }
}`

const fragmentQuery = graphql`
  fragment TopicPage_topic on Topic @argumentDefinitions(
    searchString: {type: "String", defaultValue: ""},
  ) {
    displayName
    id
    showRepoOwnership

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

    children(first: 1000, searchString: $searchString) @connection(key: "Topic_children") {
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

function renderTopicChild(view: ViewType, child: TopicChildType | null) {
  if (!child) return null

  if (child.__typename == 'Topic') {
    return (
      <Topic
        key={child.id}
        topic={child}
        viewerId={view.viewer.id}
      />
    )
  }

  if (child.__typename == 'Link') {
    return (
      <Link
        key={child.id}
        link={child}
        viewer={view.viewer}
      />
    )
  }

  return null
}

function renderRepoOwnership(topic: TopicType) {
  const repoColors = topic.repoTopics.map((repoTopic) => repoTopic.displayColor as Color)

  return (
    <RepoOwnership
      showRepoOwnership={topic.showRepoOwnership}
      repoColors={repoColors}
    />
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

function renderTopicViews(topic: TopicType, isGuest: boolean) {
  const recentActivityLocation = {
    pathname: `${topicPath(topic.id)}/recent`,
    query: {},
    search: '',
    state: {
      itemTitle: topic.displayName,
    },
  }

  const linksToReviewLocation = {
    pathname: `${topicPath(topic.id)}/review`,
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
        {!isGuest && (
          <li className="Box-row">
            <FoundLink to={linksToReviewLocation} className="Box-row-link">
              Links to review
            </FoundLink>
          </li>
        )}
      </ul>
    </div>
  )
}

export default function TopicPage(props: Props) {
  const topic = useFragment(fragmentQuery, props.topic)

  const renderHeadingDetail = useCallback(() => headingDetail(topic), [headingDetail, topic])
  const children = liftNodes<TopicChildType>(topic.children)
  const view = props.view
  const isGuest = view.viewer.isGuest

  if (!topic) {
    return (
      <div>
        Topic not found:
        {location.pathname}
      </div>
    )
  }

  return (
    <Page>
      <Subhead
        heading={topic.displayName}
        renderHeadingDetail={renderHeadingDetail}
      />
      <Columns>
        <RightColumn>
          <SidebarList
            items={liftNodes<ParentTopicType>(topic.displayParentTopics)}
            placeholder="There are no parent topics for this topic."
            title="Parent topics"
          />
          {renderTopicViews(topic, isGuest)}
          {isGuest
            ? renderNotification()
            : <AddForm topic={topic} viewer={props.view.viewer} />}
        </RightColumn>
        <LeftColumn>
          <List
            placeholder="There are no items in this list."
            hasItems={!isEmpty(children)}
          >
            {children.filter(Boolean).map((child) => renderTopicChild(view, child))}
          </List>
        </LeftColumn>
      </Columns>
    </Page>
  )
}
