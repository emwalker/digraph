import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
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
import { TopicPage_topic$data as TopicType } from '__generated__/TopicPage_topic.graphql'
import AddForm from './AddForm'

type ViewType = Response['view']
type TopicChildType = NodeTypeOf<TopicType['children']>
type ParentTopicType = NodeTypeOf<TopicType['displayParentTopics']>

type Props = {
  // eslint-disable-next-line react/no-unused-prop-types
  alerts: Object[],
  location: LocationType,
  orgLogin: string,
  topic: TopicType,
  view: ViewType,
}

type State = {}

class TopicPage extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {}
  }

  static getDerivedStateFromProps = (nextProps: Props) => {
    const shouldAppend = window.flashMessages && nextProps.alerts && nextProps.alerts.length > 0
    if (shouldAppend && window.flashMessages?.addMessage)
      // eslint-disable-next-line nonblock-statement-body-position
      nextProps.alerts.forEach(window.flashMessages.addMessage)

    return {}
  }

  get children() {
    return liftNodes<TopicChildType>(this.props.topic.children)
  }

  get displaySynonyms() {
    return this.props.topic.displaySynonyms
  }

  get isGuest(): boolean {
    return this.props.view.viewer.isGuest
  }

  get recentActivityLocation(): LocationType {
    return {
      pathname: `${topicPath(this.props.topic.id)}/recent`,
      query: {},
      search: '',
      state: {
        itemTitle: this.props.topic.displayName,
      },
    }
  }

  get linksToReviewLocation(): LocationType {
    return {
      pathname: `${topicPath(this.props.topic.id)}/review`,
      query: {},
      search: '',
      state: {
        itemTitle: this.props.topic.displayName,
      },
    }
  }

  renderTopicChild = (child: TopicChildType | null) => {
    if (!child) return null

    if (child.__typename == 'Topic') {
      return (
        <Topic
          key={child.id}
          topic={child}
        />
      )
    }

    if (child.__typename == 'Link') {
      return (
        <Link
          key={child.id}
          link={child}
          viewer={this.props.view.viewer}
        />
      )
    }

    return null
  }

  renderAddForm = () => (
    <AddForm topic={this.props.topic} viewer={this.props.view.viewer} />
  )

  get repoColors(): Color[] {
    return this.props.topic.repoTopics.map((repoTopic) => repoTopic.displayColor as Color)
  }

  renderRepoOwnership = () => 
    <RepoOwnership
      showRepoOwnership={this.props.topic.showRepoOwnership}
      repoColors={this.repoColors}
    />

  renderHeadingDetail = () => {
    const { displaySynonyms } = this
    const { length } = displaySynonyms

    if (length < 2) return <div>{ this.renderRepoOwnership() }</div>

    return (
      <div>
        <div className="displaySynonyms h6">
          {displaySynonyms.map(({ name }) => (
            <span key={name} className="synonym">{name}</span>
          ))}
        </div>

        { this.renderRepoOwnership() }
      </div>
    )
  }

  renderNotification = () => (
    <div className="Box p-3 mt-3">
      You must be
      {' '}
      <a href="/login">signed in</a>
      {' '}
      to add and move topics and links.
    </div>
  )

  renderTopicViews = () => (
    <div className="Box Box--condensed mb-3">
      <div className="Box-header">
        <span className="Box-title">This topic</span>
      </div>
      <ul>
        <li className="Box-row">
          <FoundLink to={this.recentActivityLocation} className="Box-row-link">
            Recent activity
          </FoundLink>
        </li>
        {!this.isGuest && (
          <li className="Box-row">
            <FoundLink to={this.linksToReviewLocation} className="Box-row-link">
              Links to review
            </FoundLink>
          </li>
        )}
      </ul>
    </div>
  )

  render = () => {
    const { location, topic } = this.props

    if (!topic) {
      return (
        <div>
          Topic not found:
          {location.pathname}
        </div>
      )
    }

    const { displayName, displayParentTopics } = topic
    const children = this.children.filter(Boolean)

    return (
      <Page>
        <Subhead
          heading={displayName}
          renderHeadingDetail={this.renderHeadingDetail}
        />
        <Columns>
          <RightColumn>
            <SidebarList
              items={liftNodes<ParentTopicType>(displayParentTopics)}
              placeholder="There are no parent topics for this topic."
              title="Parent topics"
            />
            {this.renderTopicViews()}
            {this.isGuest
              ? this.renderNotification()
              : this.renderAddForm()}
          </RightColumn>
          <LeftColumn>
            <List
              placeholder="There are no items in this list."
              hasItems={!isEmpty(children)}
            >
              {children.map(this.renderTopicChild)}
            </List>
          </LeftColumn>
        </Columns>
      </Page>
    )
  }
}

export const UnwrappedTopicPage = TopicPage

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

export default createFragmentContainer(TopicPage, {
  topic: graphql`
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
  `,
})
