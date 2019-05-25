// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import { liftNodes } from 'utils'
import type { LinkType, Relay, TopicType, UserType, ViewType } from 'components/types'
import Subhead from 'components/ui/Subhead'
import Breadcrumbs from 'components/ui/Breadcrumbs'
import SidebarList from 'components/ui/SidebarList'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import AddForm from './AddForm'

type Props = {
  // eslint-disable-next-line react/no-unused-prop-types
  alerts: Object[],
  location: Object,
  orgLogin: string,
  relay: Relay,
  router: Object,
  topic: TopicType,
  view: ViewType,
  viewer: UserType,
}

type State = {}

class TopicPage extends Component<Props, State> {
  static getDerivedStateFromProps = (nextProps: Props) => {
    if (window.flashMessages && nextProps.alerts && nextProps.alerts.length > 0)
      nextProps.alerts.forEach(window.flashMessages.addMessage)
    return {}
  }

  state = {}

  get links(): LinkType[] {
    return liftNodes(this.props.topic.links)
  }

  get topics(): TopicType[] {
    return liftNodes(this.props.topic.childTopics)
  }

  renderLink = (link: LinkType) => (
    <Link
      key={link.id}
      link={link}
      orgLogin={this.props.orgLogin}
      relay={this.props.relay}
      view={this.props.view}
      viewer={this.props.viewer}
    />
  )

  renderTopic = (topic: TopicType) => (
    <Topic
      key={topic.id}
      orgLogin={this.props.orgLogin}
      relay={this.props.relay}
      topic={topic}
      view={this.props.view}
      viewer={this.props.viewer}
    />
  )

  renderAddForm = () => (
    <AddForm
      relay={this.props.relay}
      topic={this.props.topic}
      viewer={this.props.viewer}
    />
  )

  renderNotification = () => (
    <div className="Box p-3 mt-3">
      You must be <a href="/login">signed in</a> to add and move topics and links.
    </div>
  )

  render = () => {
    const { location, topic, view } = this.props

    if (!topic)
      return <div>Topic not found: {location.pathname}</div>

    const { name, parentTopics, resourcePath } = topic
    const { topics, links } = this
    const { currentRepository } = view

    return (
      <div className="px-3 px-md-6 px-lg-0">
        <Breadcrumbs
          orgLogin={this.props.orgLogin}
          repository={currentRepository}
        />
        <Subhead
          heading={name}
          headingLink={resourcePath}
          location={this.props.location}
          orgLogin={this.props.orgLogin}
          router={this.props.router}
          view={this.props.view}
        />
        <Columns>
          <RightColumn>
            <SidebarList
              title="Parent topics"
              orgLogin={this.props.orgLogin}
              repoName={currentRepository.name}
              items={liftNodes(parentTopics)}
            />
            { this.props.viewer.isGuest
              ? this.renderNotification()
              : this.renderAddForm()
            }
          </RightColumn>
          <LeftColumn>
            <List
              placeholder="There are no items in this list."
              hasItems={!isEmpty(topics) || !isEmpty(links)}
            >
              { topics.map(this.renderTopic) }
              { links.map(this.renderLink) }
            </List>
          </LeftColumn>
        </Columns>
      </div>
    )
  }
}

export const query = graphql`
query TopicPage_query_Query(
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $topicId: ID!,
  $searchString: String,
) {
  alerts {
    id
    text
    type
  }

  viewer {
    id
    isGuest
    ...AddForm_viewer
    ...Link_viewer
    ...Topic_viewer
  }

  view(
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    currentRepository {
      displayName
      ...Breadcrumbs_repository
    }

    ...Link_view
    ...Topic_view
    ...Subhead_view

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
    resourcePath
    ...AddForm_topic

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
