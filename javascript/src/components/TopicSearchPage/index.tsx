import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
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
  TopicSearchPage_query_QueryResponse as Response,
} from '__generated__/TopicSearchPage_query_Query.graphql'
import { TopicSearchPage_topic as TopicType } from '__generated__/TopicSearchPage_topic.graphql'

type ViewType = Response['view']
type ParentTopicType = NodeTypeOf<TopicType['displayParentTopics']>
type SearchItemType = NodeTypeOf<TopicType['search']>

/* eslint no-underscore-dangle: 0 */

type Props = {
  orgLogin: string,
  topic: TopicType,
  view: ViewType,
}

class TopicSearchPage extends Component<Props> {
  renderSearchResultItem = (item: any) => {
    if (item.__typename === 'Link') {
      return (
        <Link
          key={item.id}
          link={item}
          viewer={this.props.view.viewer}
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
    const { topic } = this.props
    if (topic == null) return <div>Error parsing route</div>

    const {
      search: searchResults,
      displayName,
      displayParentTopics,
    } = topic
    const rows = liftNodes<SearchItemType>(searchResults)

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
                hasItems={!isEmpty(rows)}
              >
                { rows.map(this.renderSearchResultItem) }
              </List>
            </LeftColumn>
          </Columns>
        </div>
      </Page>
    )
  }
}

export const query = graphql`
query TopicSearchPage_query_Query(
  $viewerId: ID!,
  $repoIds: [ID!],
  $topicId: String!,
  $searchString: String!,
) {
  view(
    viewerId: $viewerId,
    repositoryIds: $repoIds,
  ) {
    viewer {
      ...Link_viewer
    }

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
