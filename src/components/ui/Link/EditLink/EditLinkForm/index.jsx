// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LinkType, OrganizationType } from 'components/types'
import Input from 'components/ui/Input'
import upsertLinkMutation from 'mutations/upsertLinkMutation'
import updateLinkTopicsMutation from 'mutations/updateLinkTopicsMutation'
import EditTopicList from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'
import { liftNodes } from 'utils'

type Props = {
  id: string,
  isOpen: boolean,
  relay: {
    environment: Object,
  },
  link: LinkType,
  organization: OrganizationType,
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
    upsertLinkMutation(
      this.props.relay.environment,
      configs,
      {
        addParentTopicIds: [],
        organizationId: this.props.organization.id,
        title: this.state.title,
        url: this.state.url,
      },
    )
    this.props.toggleForm()
  }

  get availableTopics(): Object[] {
    return liftNodes(this.props.link.availableTopics)
  }

  get selectedTopics(): string[] {
    return liftNodes(this.props.link.selectedTopics)
  }

  updateTitle = (event: Object) => {
    this.setState({ title: event.currentTarget.value })
  }

  updateTopics = (parentTopicIds: string[]) => {
    updateLinkTopicsMutation(
      this.props.relay.environment,
      [],
      {
        linkId: this.props.link.id,
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
            id={`edit-link-title-${this.props.id}`}
            label="Page title"
            onChange={this.updateTitle}
            value={this.state.title}
          />
          <Input
            className="col-6"
            id={`edit-link-url-${this.props.id}`}
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
          availableTopics={this.availableTopics}
          selectedTopics={this.selectedTopics}
          updateTopics={this.updateTopics}
        />
      </div>
    )
  }
}

export default createFragmentContainer(EditLinkForm, graphql`
  fragment EditLinkForm_organization on Organization {
    id
  }

  fragment EditLinkForm_link on Link {
    id
    title
    url

    selectedTopics: parentTopics(first: 1000) {
      edges {
        node {
          id
          name
        }
      }
    }

    availableTopics: availableParentTopics(first: 1000) {
      edges {
        node {
          id
          name
        }
      }
    }
  }
`)
