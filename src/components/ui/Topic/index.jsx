// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import classNames from 'classnames'

import type { Relay, TopicType, UserType, ViewType } from '../../types'
import { liftNodes } from '../../../utils'
import Item from '../Item'
import EditTopic from './EditTopicContainer'
import { topicRow } from './styles.module.css'

type Props = {
  orgLogin: string,
  relay: Relay,
  topic: TopicType,
  view: ViewType,
  viewer: UserType,
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
    if (!this.repo) return true
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

  get showEditButton(): boolean {
    return !this.props.topic.loading && !this.props.viewer.isGuest
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render = () => {
    const { topic } = this.props

    return (
      <Item
        canEdit={!this.props.viewer.isGuest}
        className={classNames(topicRow, 'Box-row--topic')}
        description={this.props.topic.description}
        displayColor={this.displayColor}
        formIsOpen={this.state.formIsOpen}
        newlyAdded={this.props.topic.newlyAdded}
        orgLogin={this.props.orgLogin}
        repoName={topic.repository && topic.repository.name}
        showEditButton={this.showEditButton}
        showLink={false}
        title={topic.displayName}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={topic.resourcePath}
      >
        <EditTopic
          isOpen={this.state.formIsOpen}
          orgLogin={this.props.orgLogin}
          relay={this.props.relay}
          toggleForm={this.toggleForm}
          topic={topic}
          view={this.props.view}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, {
  view: graphql`
    fragment Topic_view on View {
      currentRepository {
        id
      }
    }
  `,
  viewer: graphql`
    fragment Topic_viewer on User {
      isGuest
    }
  `,
  topic: graphql`
    fragment Topic_topic on Topic {
      description
      displayName
      id
      loading
      newlyAdded
      resourcePath

      repository {
        name
        displayColor
        id
      }

      parentTopics(first: 100) {
        edges {
          node {
            displayName
            resourcePath
          }
        }
      }
    }
  `,
})
