// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import classNames from 'classnames'

import { TopicType } from 'components/types'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import SelectRepository from './SelectRepository'
import './index.css'

type Props = {
  topic: TopicType,
  viewer: Object,
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
    return repo && repo.isPrivate
  }

  render = () => (
    <form className={this.className}>
      <SelectRepository
        viewer={this.props.viewer}
      />
      <AddTopic
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
      <AddLink
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
