// @flow
import React, { Component } from 'react'
import { createRefetchContainer, graphql } from 'react-relay'

import type { Option, Relay } from 'components/types'
import Input from 'components/ui/Input'
import deleteLinkMutation, { type Input as DeleteInput } from 'mutations/deleteLinkMutation'
import upsertLinkMutation, { type Input as UpsertInput } from 'mutations/upsertLinkMutation'
import updateLinkTopicsMutation, { type Input as UpdateTopicsInput } from 'mutations/updateLinkTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'
import DeleteButton from 'components/ui/DeleteButton'
import type { EditLinkForm_link as LinkType } from './__generated__/EditLinkForm_link.graphql'

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
    const { name, organization: { login } } = this.props.link.repository

    const input: UpsertInput = {
      addParentTopicIds: [],
      organizationLogin: login,
      repositoryName: name,
      title: this.state.title,
      url: this.state.url,
    }

    upsertLinkMutation(this.props.relay.environment, input)
    this.props.toggleForm()
  }

  onDelete = () => {
    const input: DeleteInput = { linkId: this.props.link.id }
    deleteLinkMutation(
      this.props.relay.environment,
      input,
      {
        configs: [{
          type: 'NODE_DELETE',
          deletedIDFieldName: 'deletedLinkId',
        }],
      },
    )
  }

  get linkId(): string {
    return this.props.link.id
  }

  get selectedTopics(): ?Option[] {
    const { link } = this.props
    return link.selectedTopics && makeOptions(link.selectedTopics)
  }

  updateTitle = (event: Object) => {
    this.setState({ title: event.currentTarget.value })
  }

  updateTopics = (parentTopicIds: string[]) => {
    const input: UpdateTopicsInput = {
      linkId: this.linkId,
      parentTopicIds,
    }
    updateLinkTopicsMutation(this.props.relay.environment, input)
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
        const { availableTopics } = this.props.link
        const options = availableTopics ? makeOptions(availableTopics) : []
        resolve(options)
      })
    })
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  render() {
    const { selectedTopics } = this
    if (!this.props.isOpen) return null

    return (
      selectedTopics ? (
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
            selectedTopics={selectedTopics}
            updateTopics={this.updateTopics}
          />
        </div>
      ) : null
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

      selectedTopics: parentTopics(first: 1000) {
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
    $viewerId: ID!,
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
    $linkId: ID!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
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
