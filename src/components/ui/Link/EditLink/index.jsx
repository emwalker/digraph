// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import Input from '../../Input'
import upsertLinkMutation from '../../../../mutations/upsertLinkMutation'

type Props = {
  id: string,
  isOpen: boolean,
  relay: {
    environment: Object,
  },
  link: {
    resourceId: string,
    title: string,
    url: string,
  },
  organization: {
    resourceId: string,
  },
  toggleForm: Function,
}

type State = {
  title: string,
  url: string,
}

class EditLink extends Component<Props, State> {
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
        organizationId: this.props.organization.resourceId,
        resourceId: this.props.link.resourceId,
        title: this.state.title,
        url: this.state.url,
      },
    )
    this.props.toggleForm()
  }

  updateTitle = (event: Object) => {
    this.setState({ title: event.currentTarget.value })
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
        <div>
          <button onClick={this.onSave} className="btn-primary">Save</button>
          {' '} or {' '}
          <button onClick={this.props.toggleForm} className="btn-link">cancel</button>
        </div>
      </div>
    )
  }
}

export default createFragmentContainer(EditLink, graphql`
  fragment EditLink_organization on Organization {
    resourceId
  }

  fragment EditLink_link on Link {
    resourceId
    title
    url
  }
`)
