// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { Relay } from 'components/types'
import upsertTopicMutation from 'mutations/upsertTopicMutation'
import type { AddTopic_viewer as Viewer } from './__generated__/AddTopic_viewer.graphql'
import type { AddTopic_topic as Topic } from './__generated__/AddTopic_topic.graphql'

type Repository = $PropertyType<Viewer, 'selectedRepository'>

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  relay: Relay,
  topic: Topic,
  viewer: Viewer,
}

type State = {
  name: string,
}

class AddTopic extends Component<Props, State> {
  static defaultProps = {
    disabled: true,
  }

  state = {
    name: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter') this.createTopic()
  }

  get selectedRepo(): ?Repository {
    return this.props.viewer.selectedRepository
  }

  get orgLogin(): ?string {
    const repo = this.selectedRepo
    if (!repo) return null
    return repo.organization.login
  }

  get relayConfigs() {
    return [{
      type: 'RANGE_ADD',
      parentID: this.props.topic.id,
      connectionInfo: [{
        key: 'Topic_childTopics',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'topicEdge',
    }]
  }

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  createTopic() {
    const repo = this.selectedRepo
    const repoName = repo ? repo.name : null

    upsertTopicMutation(
      this.props.relay.environment,
      {
        name: this.state.name,
        repositoryName: repoName,
        organizationLogin: this.orgLogin,
        topicIds: [this.props.topic.id],
      },
      {
        configs: this.relayConfigs,
      },
    )
    this.setState({ name: '' })
  }

  render = () => (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-ne"
          aria-label={tooltipText}
        >
          <label htmlFor="create-topic-name">Add subtopic</label>
        </span>
      </dt>
      <dd>
        <input
          className="form-control test-topic-name input-sm"
          disabled={this.props.disabled}
          id="create-topic-name"
          onChange={this.updateName}
          onKeyPress={this.onKeyPress}
          placeholder="Name or description"
          value={this.state.name}
        />
      </dd>
    </dl>
  )
}

export default createFragmentContainer(AddTopic, {
  viewer: graphql`
    fragment AddTopic_viewer on User {
      selectedRepository {
        name
        organization {
          login
        }
      }
    }
  `,
  topic: graphql`
    fragment AddTopic_topic on Topic {
      id
    }
  `,
})
