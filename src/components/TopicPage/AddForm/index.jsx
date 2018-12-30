// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import classNames from 'classnames'

import type { Relay, TopicType, UserType } from 'components/types'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import SelectRepository from './SelectRepository'
import './index.css'

type Props = {
  relay: Relay,
  topic: TopicType,
  viewer: UserType,
}

class AddForm extends Component<Props> {
  get className(): string {
    return classNames(
      'border',
      'rounded-1',
      'p-2',
      'mt-3',
      { 'private-repo': this.isPrivateRepo },
    )
  }

  get isPrivateRepo(): boolean {
    const repo = this.props.viewer.selectedRepository
    if (!repo)
      return true
    return repo.isPrivate
  }

  render = () => (
    <form className={this.className}>
      <SelectRepository
        relay={this.props.relay}
        viewer={this.props.viewer}
      />
      <AddTopic
        relay={this.props.relay}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
      <AddLink
        relay={this.props.relay}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
    </form>
  )
}

export default createFragmentContainer(AddForm, graphql`
  fragment AddForm_viewer on User {
    selectedRepository {
      isPrivate
    }

    ...AddLink_viewer
    ...AddTopic_viewer
    ...SelectRepository_viewer
  }

  fragment AddForm_topic on Topic {
    ...AddLink_topic
    ...AddTopic_topic
  }
`)
