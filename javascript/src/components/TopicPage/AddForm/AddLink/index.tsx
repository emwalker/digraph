import React, { Component, KeyboardEvent, FormEvent } from 'react'
import { graphql, createFragmentContainer, RelayProp } from 'react-relay'

import upsertLinkMutation, { Input } from 'mutations/upsertLinkMutation'
import { AddLink_viewer$data as ViewerType } from '__generated__/AddLink_viewer.graphql'
import { AddLink_topic$data as TopicType } from '__generated__/AddLink_topic.graphql'

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
    if (event.key === 'Enter') this.upsertLink()
  }

  get selectedRepo(): RepositoryType {
    return this.props.viewer.selectedRepository
  }

  updateUrl = (event: FormEvent<HTMLInputElement>) => {
    this.setState({ url: event.currentTarget.value })
  }

  upsertLink() {
    const repoId = this.props.viewer.selectedRepository?.id

    if (!repoId) return

    const input: Input = {
      addParentTopicId: this.props.topic.id,
      repoId,
      url: this.state.url,
    }

    upsertLinkMutation(this.props.relay.environment, input)
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
      }
    }
  `,
  topic: graphql`
    fragment AddLink_topic on Topic {
      id
    }
  `,
})
