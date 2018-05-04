import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

type Props = {
  topic: {
    name: string,
  }
}

const TopicPage = ({ topic: { name } }: Props) => (
  <div>
    <h1>Topic: {name}</h1>
  </div>
)

export const query = graphql`
query TopicPage_query_Query(
  $organizationId: String!,
  $topicId: String!
) {
  organization(resourceId: $organizationId) {
    topic(resourceId: $topicId) {
      ...TopicPage_topic
    }
  }
}`

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_topic on Topic {
    name
  }
`)
