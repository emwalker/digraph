import React, { Component, KeyboardEvent, FormEvent } from 'react'
import { graphql, createFragmentContainer, RelayProp, DeclarativeMutationConfig } from 'react-relay'

import upsertTopicMutation from 'mutations/upsertTopicMutation'
import { AddTopic_viewer as Viewer } from '__generated__/AddTopic_viewer.graphql'
import { AddTopic_topic as Topic } from '__generated__/AddTopic_topic.graphql'

type RepositoryType = Viewer['selectedRepository']

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  relay: RelayProp,
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

  constructor(props: Props) {
    super(props)
    this.state = {
      name: '',
    }
  }

  onKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') this.createTopic()
  }

  get selectedRepo(): RepositoryType {
    return this.props.viewer.selectedRepository
  }

  get relayConfigs(): DeclarativeMutationConfig[] {
    return [{
      type: 'RANGE_ADD',
      parentID: this.props.topic.id,
      connectionInfo: [{
        key: 'Topic_children',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'topicEdge',
    }]
  }

  updateName = (event: FormEvent<HTMLInputElement>) => {
    this.setState({ name: event.currentTarget.value })
  }

  createTopic() {
    const repo = this.selectedRepo
    if (!repo) {
      // eslint-disable-next-line no-console
      console.log('missing repo')
      return
    }

    upsertTopicMutation(
      this.props.relay.environment,
      {
        name: this.state.name,
        repoPrefix: repo.prefix,
        parentTopicPath: this.props.topic.path,
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
        prefix
      }
    }
  `,
  topic: graphql`
    fragment AddTopic_topic on Topic {
      path
      id
    }
  `,
})
