// @flow
import React, { Component } from 'react'
import { createRefetchContainer, graphql } from 'react-relay'

import type { LinkType, Option, Relay } from 'components/types'
import Input from 'components/ui/Input'
import deleteLinkMutation from 'mutations/deleteLinkMutation'
import upsertLinkMutation from 'mutations/upsertLinkMutation'
import updateLinkTopicsMutation from 'mutations/updateLinkTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'
import DeleteButton from 'components/ui/DeleteButton'

type Props = {
  isOpen: boolean,
  link: LinkType,
  orgLogin: string,
  relay: Relay,
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

  onDelete = () => {
    deleteLinkMutation(
      this.props.relay.environment,
      [{
        type: 'NODE_DELETE',
        deletedIDFieldName: 'deletedLinkId',
      }],
      {
        linkId: this.props.link.id,
      },
    )
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

  loadOptions = (searchString: string): Promise<Option[]> => {
    if (!this.props.relay) return new Promise(() => [])

    return new Promise((resolve) => {
      const variables = {
        orgLogin: this.props.orgLogin,
        count: 40,
        searchString,
      }

      this.props.relay.refetch(variables, null, () => {
        const options = makeOptions(this.props.link.availableTopics)
        resolve(options)
      })
    })
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  render() {
    if (!this.props.isOpen) return null

    return (
      <div>
        <Input
          className="col-12"
          id={`edit-link-title-${this.linkId}`}
          label="Page title"
          onChange={this.updateTitle}
          value={this.state.title}
        />
        <Input
          className="col-12"
          id={`edit-link-url-${this.linkId}`}
          label="Url"
          onChange={this.updateUrl}
          value={this.state.url}
        />
        <div>
          <SaveOrCancel
            onSave={this.onSave}
            onCancel={this.props.toggleForm}
          />
          <DeleteButton
            className="float-right"
            onDelete={this.onDelete}
          />
        </div>
        <EditTopicList
          loadOptions={this.loadOptions}
          selectedTopics={makeOptions(this.props.link.selectedTopics)}
          updateTopics={this.updateTopics}
        />
      </div>
    )
  }
}

export default createRefetchContainer(EditLinkForm, {
  link: graphql`
    fragment EditLinkForm_link on Link @argumentDefinitions(
      searchString: {type: "String", defaultValue: null},
      count: {type: "Int!", defaultValue: 10}
    ) {
      id
      title
      url

      repository {
        name

        organization {
          login
        }
      }

      selectedTopics: parentTopics(first: 10) {
        edges {
          node {
            value: id
            label: displayName
          }
        }
      }

      availableTopics: availableParentTopics(searchString: $searchString, first: $count) {
        edges {
          node {
            value: id
            label: displayName
          }
        }
      }
    }
  `,
},
graphql`
  query EditLinkFormRefetchQuery(
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
    $linkId: ID!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      link(id: $linkId) {
        ...EditLinkForm_link @arguments(count: $count, searchString: $searchString)
      }
    }
  }
`)
