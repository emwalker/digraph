// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { TopicType } from '../../types'
import { liftNodes } from '../../../utils'
import Item from '../Item'

type Props = {
  topic: TopicType,
}

type State = {
  formIsOpen: boolean,
}

class Topic extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.parentTopics = liftNodes(props.topic.parentTopics)
    this.state = {
      formIsOpen: false,
    }
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render() {
    return (
      <Item
        className="Box-row--topic"
        formIsOpen={this.state.formIsOpen}
        title={this.props.topic.name}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={this.props.topic.resourcePath}
      >
        Edit topic
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, graphql`
  fragment Topic_organization on Organization {
    id
  }

  fragment Topic_topic on Topic {
    name
    resourcePath

    parentTopics(first: 10) {
      edges {
        node {
          name
          resourcePath
        }
      }
    }
  }
`)
