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

export default createFragmentContainer(TopicPage, graphql`
  fragment TopicPage_topic on Topic {
    name
  }
`)
