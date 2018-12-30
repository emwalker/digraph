// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { Relay, TopicType, ViewType } from '../../types'
import { liftNodes } from '../../../utils'
import Item from '../Item'
import EditTopic from './EditTopic'

type Props = {
  orgLogin: string,
  relay: Relay,
  topic: TopicType,
  view: ViewType,
}

type State = {
  formIsOpen: boolean,
}

class Topic extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get repo(): ?Object {
    return this.props.topic.repository
  }

  get currentRepo(): Object {
    return this.props.view.currentRepository
  }

  get topicBelongsToCurrentRepo(): boolean {
    if (!this.repo)
      return true
    return this.repo.id === this.currentRepo.id
  }

  get displayColor(): string {
    return this.topicBelongsToCurrentRepo
      ? 'transparent'
      : this.props.topic.repository.displayColor
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
        description={this.props.topic.description}
        displayColor={this.displayColor}
        formIsOpen={this.state.formIsOpen}
        newlyAdded={this.props.topic.newlyAdded}
        title={this.props.topic.name}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={this.props.topic.resourcePath}
      >
        <EditTopic
          isOpen={this.state.formIsOpen}
          orgLogin={this.props.orgLogin}
          relay={this.props.relay}
          toggleForm={this.toggleForm}
          topic={this.props.topic}
          view={this.props.view}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, graphql`
  fragment Topic_view on View {
    currentRepository {
      id
    }
  }

  fragment Topic_topic on Topic {
    description
    id
    name
    newlyAdded
    resourcePath

    repository {
      displayColor
      id
    }

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
