// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { Relay, RepositoryType, TopicType, UserType } from 'components/types'
import upsertLinkMutation from 'mutations/upsertLinkMutation'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = {
  disabled?: boolean,
  relay: Relay,
  topic: TopicType,
  viewer: UserType,
}

type State = {
  url: string,
}

class AddLink extends Component<Props, State> {
  static defaultProps = {
    disabled: true,
  }

  state = {
    url: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createLink()
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
        key: 'Topic_links',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'linkEdge',
    }]
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  createLink() {
    const repo = this.selectedRepo
    const repoName = repo ? repo.name : null

    upsertLinkMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        addParentTopicIds: [this.props.topic.id],
        organizationLogin: this.orgLogin,
        repositoryName: repoName,
        url: this.state.url,
      },
    )
    this.setState({ url: '' })
  }

  render = () => (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-w"
          aria-label="Add a link to this topic."
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

export default createFragmentContainer(AddLink, graphql`
  fragment AddLink_viewer on User {
    selectedRepository {
      id
      name

      organization {
        login
      }
    }
  }

  fragment AddLink_topic on Topic {
    id
  }
`)
