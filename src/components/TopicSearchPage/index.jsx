// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import type { LinkType, Relay, TopicType, UserType, ViewType } from 'components/types'
import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import Breadcrumbs from 'components/ui/Breadcrumbs'
import { liftNodes } from 'utils'

/* eslint no-underscore-dangle: 0 */

type Props = {
  location: Object,
  orgLogin: string,
  relay: Relay,
  router: Object,
  topic: TopicType,
  view: ViewType,
  viewer: UserType,
}

class TopicSearchPage extends Component<Props> {
  renderSearchResultItem = (item: any) => {
    if (item.__typename === 'Link') {
      const link = (item: LinkType)

      return (
        <Link
          key={link.id}
          link={link}
          orgLogin={this.props.orgLogin}
          relay={this.props.relay}
          view={this.props.view}
          viewer={this.props.viewer}
        />
      )
    }

    const topic = (item: TopicType)

    return (
      <Topic
        key={topic.id}
        orgLogin={this.props.orgLogin}
        relay={this.props.relay}
        topic={topic}
        view={this.props.view}
        viewer={this.props.viewer}
      />
    )
  }

  render = () => {
    const { location, orgLogin, router, topic, view } = this.props
    const {
      search: searchResults,
      name,
      parentTopics,
      resourcePath,
    } = topic
    const rows = liftNodes(searchResults)
    const { currentRepository: repo } = view

    return (
      <div className="px-3 px-md-6 px-lg-0">
        <Breadcrumbs
          orgLogin={orgLogin}
          repository={repo}
        />
        <Subhead
          heading={name}
          headingLink={resourcePath}
          location={location}
          orgLogin={orgLogin}
          router={router}
          view={view}
        />
        <Columns>
          <RightColumn>
            <SidebarList
              items={liftNodes(parentTopics)}
              orgLogin={this.props.orgLogin}
              repoName={repo.displayName}
              title="Parent topics"
            />
          </RightColumn>
          <LeftColumn>
            <List
              placeholder="There are no items in this list."
              hasItems={!isEmpty(rows)}
            >
              { rows.map(this.renderSearchResultItem) }
            </List>
          </LeftColumn>
        </Columns>
      </div>
    )
  }
}

export const query = graphql`
query TopicSearchPage_query_Query(
  $orgLogin: String!
  $repoName: String,
  $repoIds: [ID!],
  $topicId: ID!,
  $searchString: String!,
) {
  viewer {
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
      ...TopicSearchPage_topic @arguments(searchString: $searchString)
    }
  }
}`

export default createFragmentContainer(TopicSearchPage, {
  topic: graphql`
    fragment TopicSearchPage_topic on Topic @argumentDefinitions(
      searchString: {type: "String!", defaultValue: ""},
    ) {
      name
      resourcePath

      parentTopics(first: 100) {
        edges {
          node {
            display: name
            resourcePath
          }
        }
      }

      search(first: 100, searchString: $searchString) {
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
  `,
})
