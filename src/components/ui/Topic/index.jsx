// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { OrganizationType, TopicType } from '../../types'
import { liftNodes } from '../../../utils'
import Item from '../Item'
import EditTopic from './EditTopic'

type Props = {
  organization: OrganizationType,
  orgLogin: string,
  topic: TopicType,
  viewer: Object,
}

type State = {
  formIsOpen: boolean,
}

class Topic extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get displayColor(): string {
    return this.props.topic.belongsToCurrentRepository
      ? 'transparent'
      : this.props.topic.displayColor
  }

  get parentTopics(): TopicType[] {
    return liftNodes(this.props.topic.parentTopics)
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render() {
    return (
      <Item
        className="Box-row--topic"
        displayColor={this.displayColor}
        formIsOpen={this.state.formIsOpen}
        title={this.props.topic.name}
        description={this.props.topic.description}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={this.props.topic.resourcePath}
      >
        <EditTopic
          isOpen={this.state.formIsOpen}
          orgLogin={this.props.orgLogin}
          toggleForm={this.toggleForm}
          topic={this.props.topic}
          viewer={this.props.viewer}
          {...this.props}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, graphql`
  fragment Topic_topic on Topic {
    belongsToCurrentRepository
    description
    displayColor
    id
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
