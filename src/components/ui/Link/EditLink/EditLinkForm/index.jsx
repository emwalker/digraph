// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LinkType, Relay, TopicType } from 'components/types'
import Input from 'components/ui/Input'
import upsertLinkMutation from 'mutations/upsertLinkMutation'
import updateLinkTopicsMutation from 'mutations/updateLinkTopicsMutation'
import EditTopicList from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'

type Props = {
  availableTopics: TopicType[],
  isOpen: boolean,
  link: LinkType,
  relay: Relay,
  selectedTopics: TopicType[],
  toggleForm: Function,
}

type State = {
  title: string,
  url: string,
}

class EditLinkForm extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      title: props.link.title,
      url: props.link.url,
    }
  }

  onSave = () => {
    const configs = []
    const { name, organization: { login } } = this.props.link.repository

    upsertLinkMutation(
      this.props.relay.environment,
      configs,
      {
        addParentTopicIds: [],
        organizationLogin: login,
        repositoryName: name,
        title: this.state.title,
        url: this.state.url,
      },
    )
    this.props.toggleForm()
  }

  get linkId(): string {
    return this.props.link.id
  }

  updateTitle = (event: Object) => {
    this.setState({ title: event.currentTarget.value })
  }

  updateTopics = (parentTopicIds: string[]) => {
    updateLinkTopicsMutation(
      this.props.relay.environment,
      [],
      {
        linkId: this.linkId,
        parentTopicIds,
      },
    )
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  render() {
    if (!this.props.isOpen)
      return null

    return (
      <div>
        <div className="d-flex col-12">
          <Input
            className="col-5 mr-3"
            id={`edit-link-title-${this.linkId}`}
            label="Page title"
            onChange={this.updateTitle}
            value={this.state.title}
          />
          <Input
            className="col-6"
            id={`edit-link-url-${this.linkId}`}
            label="Url"
            onChange={this.updateUrl}
            value={this.state.url}
          />
        </div>
        <SaveOrCancel
          onSave={this.onSave}
          onCancel={this.props.toggleForm}
        />
        <EditTopicList
          availableTopics={this.props.availableTopics}
          selectedTopics={this.props.selectedTopics}
          updateTopics={this.updateTopics}
        />
      </div>
    )
  }
}

export default createFragmentContainer(EditLinkForm, graphql`
  fragment EditLinkForm_link on Link {
    id
    title
    url

    repository {
      name

      organization {
        login
      }
    }
  }
`)
