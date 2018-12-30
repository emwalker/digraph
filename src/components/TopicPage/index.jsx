// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import { liftNodes } from 'utils'
import type { TopicType } from 'components/types'
import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import Breadcrumbs from 'components/ui/Breadcrumbs'
import AddForm from './AddForm'

type Props = {
  location: Object,
  orgLogin: string,
  repoName: ?string,
  router: Object,
  topic: TopicType,
  view: {
    currentRepository: Object,
  },
  viewer: {
    id: string,
  },
}

class TopicPage extends Component<Props> {
  get links(): Object[] {
    return liftNodes(this.props.topic.links)
  }

  get topics(): Object[] {
    return liftNodes(this.props.topic.childTopics)
  }

  render = () => {
    const { location, topic, ...props } = this.props
    const { name, parentTopics, resourcePath } = topic
    const { topics, links } = this

    return (
      <div>
        <Breadcrumbs
          orgLogin={this.props.orgLogin}
          repository={this.props.view.currentRepository}
        />
        <Subhead
          heading={name}
          headingLink={resourcePath}
          location={this.props.location}
          router={this.props.router}
          view={this.props.view}
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
          <AddForm
            topic={topic}
            viewer={this.props.viewer}
          />
        </div>
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
  viewer {
    id
    ...AddForm_viewer
  }

  view(
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    currentRepository {
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
