// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import autosize from 'autosize'

import type { Relay, RepositoryType, TopicType, UserType } from 'components/types'
import upsertTopicMutation from 'mutations/upsertTopicMutation'

/* eslint jsx-a11y/label-has-for: 0 */

const tooltipText =
  'Add a subtopic to this topic. You can click "Edit"\n' +
  'afterwards if it also belongs under another topic.'

type Props = {
  disabled?: boolean,
  relay: Relay,
  topic: TopicType,
  viewer: UserType,
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

  componentDidMount() {
    autosize(this.textarea)
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createTopic()
  }

  get selectedRepo(): ?RepositoryType {
    return this.props.viewer.selectedRepository
  }

  get orgLogin(): ?string {
    const repo = this.selectedRepo
    if (!repo)
      return null
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

  textarea: ?HTMLTextAreaElement

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  createTopic() {
    const repo = this.selectedRepo
    const repoName = repo ? repo.name : null

    upsertTopicMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        name: this.state.name,
        repositoryName: repoName,
        organizationLogin: this.orgLogin,
        topicIds: [this.props.topic.id],
      },
    )
    this.setState({ name: '' })
  }

  render = () => (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-w"
          aria-label={tooltipText}
        >
          <label htmlFor="create-topic-name">Add subtopic</label>
        </span>
      </dt>
      <dd>
        <textarea
          className="form-control test-topic-name input-sm"
          disabled={this.props.disabled}
          id="create-topic-name"
          onChange={this.updateName}
          onKeyPress={this.onKeyPress}
          placeholder="Name or description"
          ref={(r) => { this.textarea = r }}
          rows={1}
          value={this.state.name}
        />
      </dd>
    </dl>
  )
}

export default createFragmentContainer(AddTopic, graphql`
  fragment AddTopic_viewer on User {
    selectedRepository {
      name
      organization {
        login
      }
    }
  }

  fragment AddTopic_topic on Topic {
    id
  }
`)
