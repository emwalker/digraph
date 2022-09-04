import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import classNames from 'classnames'

import { AddForm_topic as Topic } from '__generated__/AddForm_topic.graphql'
import { AddForm_viewer as Viewer } from '__generated__/AddForm_viewer.graphql'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import SelectRepository from './SelectRepository'
import './index.css'

type Props = {
  topic: Topic,
  viewer: Viewer,
}

class AddForm extends Component<Props> {
  get className(): string {
    return classNames(
      'border',
      'rounded-1',
      'px-md-2',
      'px-3',
      'mt-3',
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
    <>
      <AddTopic
        disabled={!this.repoSelected}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
      <AddLink
        disabled={!this.repoSelected}
        topic={this.props.topic}
        viewer={this.props.viewer}
      />
    </>
  )

  get selectRepositoryStyle(): Object {
    const backgroundColor = this.isPrivateRepo ?
      this.props.viewer.selectedRepository?.displayColor :
      'transparent'
    return { backgroundColor }
  }

  render = () => (
    <form className={this.className} style={this.selectRepositoryStyle}>
      <SelectRepository
        viewer={this.props.viewer}
      />
      {this.repoSelected && this.renderInputFields()}
    </form>
  )
}

export default createFragmentContainer(AddForm, {
  viewer: graphql`
    fragment AddForm_viewer on User {
      selectedRepository {
        isPrivate
        displayColor
      }

      ...AddLink_viewer
      ...AddTopic_viewer
      ...SelectRepository_viewer
      ...SelectedRepo_viewer
    }
  `,
  topic: graphql`
    fragment AddForm_topic on Topic {
      ...AddLink_topic
      ...AddTopic_topic
    }
  `,
})
