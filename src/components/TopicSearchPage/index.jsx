// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import List from 'components/ui/List'
import Link from 'components/ui/Link'
import Topic from 'components/ui/Topic'
import Breadcrumbs from 'components/ui/Breadcrumbs'
import type { TopicType } from '../types'
import { liftNodes } from '../../utils'

/* eslint no-underscore-dangle: 0 */

type Props = {
  location: Object,
  orgLogin: string,
  router: Object,
  topic: TopicType,
  view: {
    currentRepository: Object,
  },
}

class TopicSearchPage extends Component<Props> {
  renderSearchResultItem = (item) => {
    if (item.__typename === 'Link') {
      return (
        <Link
          key={item.id}
          orgLogin={this.props.orgLogin}
          link={item}
          view={this.props.view}
        />
      )
    }

    return (
      <Topic
        key={item.id}
        orgLogin={this.props.orgLogin}
        topic={item}
        view={this.props.view}
      />
    )
  }

  render = () => {
    const { location, orgLogin, router, topic, view } = this.props
    const {
      searchResults,
      name,
      parentTopics,
      resourcePath,
    } = topic
    const rows = liftNodes(searchResults)

    return (
      <div>
        <Breadcrumbs
          orgLogin={orgLogin}
          repository={view.currentRepository}
        />
        <Subhead
          heading={name}
          headingLink={resourcePath}
          location={location}
          router={router}
          view={view}
        />
        <div className="two-thirds column pl-0">
          <List
            placeholder="There are no items in this list."
            hasItems={!isEmpty(rows)}
          >
            { rows.map(this.renderSearchResultItem) }
          </List>
        </div>
        <div className="one-third column pr-0">
          <SidebarList
            title="Parent topics"
            items={liftNodes(parentTopics)}
          />
        </div>
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
      ...TopicSearchPage_topic @arguments(searchString: $searchString)
    }
  }
}`

export default createFragmentContainer(TopicSearchPage, graphql`
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

    searchResults: search(first: 100, searchString: $searchString) {
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
`)
