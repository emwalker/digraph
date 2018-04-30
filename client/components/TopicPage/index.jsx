import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

type Props = {
  topic: {
    name: string,
  }
}

const TopicPage = ({ topic }: Props) => (
  <div>
    <h1>Topic: {topic ? topic.name : 'Nemo'}</h1>
  </div>
)

export const query = graphql`
query TopicPage_query_Query(
  $orgResourceId: String!,
  $topicResourceId: String!
) {
  organization(resourceId: $orgResourceId) {
    topic(resourceId: $topicResourceId) {
      ...TopicPage_topic
    }
  }
}`

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_topic on Topic {
    name
  }
`)
