// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import Subhead from 'components/ui/Subhead'
import SidebarList from 'components/ui/SidebarList'
import List from 'components/ui/List'
import type { TopicType } from '../types'
import { liftNodes } from '../../utils'
import Link from '../ui/Link'
import Topic from '../ui/Topic'

/* eslint no-underscore-dangle: 0 */

type Props = {
  location: Object,
  router: Object,
  topic: TopicType,
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
    const { location, router, topic } = this.props
    const {
      searchResults,
      name,
      parentTopics,
    } = topic
    const rows = liftNodes(searchResults)

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
  $repoIds: [ID!],
  $topicId: ID!,
  $searchString: String!,
) {
  view(repositoryIds: $repoIds) {
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
