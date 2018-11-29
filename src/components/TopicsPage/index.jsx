// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import List from '../ui/List'
import Topic from '../ui/Topic'
import { liftNodes } from '../../utils'

type Props = {
  view: {
    topics: Object,
  },
}

const TopicsPage = ({ view, ...props }: Props) => {
  const topics = liftNodes(view.topics)
  return (
    <div>
      <List
        placeholder="There are no topics"
        hasItems={!isEmpty(topics)}
      >
        { topics.map(topic => (
          <Topic
            key={topic.resourcePath}
            topic={topic}
            {...props}
          />
        )) }
      </List>
    </div>
  )
}

export const query = graphql`
query TopicsPage_query_Query($orgIds: [ID!]) {
  view(organizationIds: $orgIds) {
    ...TopicsPage_view
  }
}`

export default createFragmentContainer(TopicsPage, graphql`
  fragment TopicsPage_view on View {
    topics(first: 1000) @connection(key: "View_topics") {
      edges {
        node {
          resourcePath
          ...Topic_topic
        }
      }
    }
  }
`)
