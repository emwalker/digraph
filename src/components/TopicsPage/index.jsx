// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import type { Relay, ViewType } from 'components/types'
import List from 'components/ui/List'
import Topic from 'components/ui/Topic'
import { liftNodes } from 'utils'

type Props = {
  orgLogin: string,
  relay: Relay,
  view: ViewType,
}

const TopicsPage = ({ view, ...props }: Props) => {
  const topics = liftNodes(view.topics)
  return (
    <List
      placeholder="There are no topics"
      hasItems={!isEmpty(topics)}
    >
      { topics.map(topic => (
        <Topic
          key={topic.resourcePath}
          orgLogin={props.orgLogin}
          relay={props.relay}
          topic={topic}
          view={view}
        />
      )) }
    </List>
  )
}

export const query = graphql`
query TopicsPage_query_Query(
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $searchString: String,
) {
  view(
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    ...TopicsPage_view @arguments(searchString: $searchString)
  }
}`

export default createFragmentContainer(TopicsPage, graphql`
  fragment TopicsPage_view on View @argumentDefinitions(
    searchString: {type: "String", defaultValue: ""},
  ) {
    topics(first: 100, searchString: $searchString) @connection(key: "View_topics") {
      edges {
        node {
          resourcePath
          ...Topic_topic
        }
      }
    }
  }
`)
