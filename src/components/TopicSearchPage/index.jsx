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
  viewer: {
    defaultRepository: Object,
  },
  view: {
    repository: Object,
  },
}

class TopicSearchPage extends Component<Props> {
  renderSearchResultItem = (item) => {
    if (item.__typename === 'Link') {
      return (
        <Link
          key={item.id}
          link={item}
        />
      )
    }

    return (
      <Topic
        key={item.id}
        topic={item}
      />
    )
  }

  render = () => {
    const { location, orgLogin, router, topic, view, viewer } = this.props
    const {
      searchResults,
      name,
      parentTopics,
    } = topic
    const rows = liftNodes(searchResults)

    return (
      <div>
        <Breadcrumbs
          orgLogin={orgLogin}
          repository={view.repository}
        />
        <Subhead
          heading={name}
          location={location}
          router={router}
          viewer={viewer}
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
  viewer {
    ...Subhead_viewer
  }

  view(repositoryIds: $repoIds) {
    repository(organizationLogin: $orgLogin, name: $repoName) {
      ...Breadcrumbs_repository
    }

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
