// @flow
import React, { Component, Fragment } from 'react'
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
      'px-md-2',
      'px-3',
      'mt-3',
      { 'private-repo': this.isPrivateRepo },
    )
  }

  get isPrivateRepo(): boolean {
    const repo = this.props.viewer.selectedRepository
    if (!repo) return false
    return repo.isPrivate
  }

  get repoSelected(): boolean {
    return !!this.props.viewer.selectedRepository
  }

  renderInputFields = () => (
    <Fragment>
      <AddTopic
        disabled={!this.repoSelected}
        relay={this.props.relay}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
      <AddLink
        disabled={!this.repoSelected}
        relay={this.props.relay}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
    </Fragment>
  )

  render = () => (
    <form className={this.className}>
      <SelectRepository
        relay={this.props.relay}
        viewer={this.props.viewer}
      />
      { this.repoSelected && this.renderInputFields() }
    </form>
  )
}

export default createFragmentContainer(AddForm, {
  viewer: graphql`
    fragment AddForm_viewer on User {
      selectedRepository {
        isPrivate
      }

      ...AddLink_viewer
      ...AddTopic_viewer
      ...SelectRepository_viewer
    }
  `,
  topic: graphql`
    fragment AddForm_topic on Topic {
      ...AddLink_topic
      ...AddTopic_topic
    }
  `,
})
