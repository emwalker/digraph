import React, { Component, KeyboardEvent, FormEvent } from 'react'
import { graphql, createFragmentContainer, RelayProp, DeclarativeMutationConfig } from 'react-relay'

import upsertLinkMutation, { Input } from 'mutations/upsertLinkMutation'
import { AddLink_viewer as ViewerType } from '__generated__/AddLink_viewer.graphql'
import { AddLink_topic as TopicType } from '__generated__/AddLink_topic.graphql'

type RepositoryType = ViewerType['selectedRepository']

const tooltip = 'Add a link to this topic.\n'
  + 'Press "Return" to submit the new link.'

type Props = {
  disabled?: boolean,
  relay: RelayProp,
  topic: TopicType,
  viewer: ViewerType,
}

type State = {
  url: string,
}

class AddLink extends Component<Props, State> {
  static defaultProps = {
    disabled: false,
  }

  constructor(props: Props) {
    super(props)
    this.state = {
      url: '',
    }
  }

  onKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') this.createLink()
  }

  get selectedRepo(): RepositoryType {
    return this.props.viewer.selectedRepository
  }

  get orgLogin() {
    const repo = this.selectedRepo
    if (!repo) return null
    return repo.organization.login
  }

  get relayConfigs(): DeclarativeMutationConfig[] {
    return [{
      type: 'RANGE_ADD',
      parentID: this.props.topic.id,
      connectionInfo: [{
        key: 'Topic_links',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'linkEdge',
    }]
  }

  updateUrl = (event: FormEvent<HTMLInputElement>) => {
    this.setState({ url: event.currentTarget.value })
  }

  createLink() {
    const repo = this.selectedRepo
    const repoName = repo ? repo.name : null
    const { orgLogin } = this

    if (!repoName) return
    if (!orgLogin) return

    const input: Input = {
      addParentTopicIds: [this.props.topic.id],
      organizationLogin: orgLogin,
      repositoryName: repoName,
      url: this.state.url,
    }

    upsertLinkMutation(
      this.props.relay.environment,
      input,
      {
        configs: this.relayConfigs,
      },
    )
    this.setState({ url: '' })
  }

  render = () => (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-ne"
          aria-label={tooltip}
        >
          <label htmlFor="create-link-url">Add link</label>
        </span>
      </dt>
      <dd>
        <input
          className="form-control test-link-url input-sm"
          disabled={this.props.disabled}
          id="create-link-url"
          onChange={this.updateUrl}
          onKeyPress={this.onKeyPress}
          placeholder="Url"
          type="url"
          value={this.state.url}
        />
      </dd>
    </dl>
  )
}

export default createFragmentContainer(AddLink, {
  viewer: graphql`
    fragment AddLink_viewer on User {
      selectedRepository {
        id
        name

        organization {
          login
        }
      }
    }
  `,
  topic: graphql`
    fragment AddLink_topic on Topic {
      id
    }
  `,
})
