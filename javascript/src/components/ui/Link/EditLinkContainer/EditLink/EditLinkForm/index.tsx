import React, { Component, FormEvent } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { liftNodes, NodeTypeOf } from 'components/types'
import Input from 'components/ui/Input'
import deleteLinkMutation, { Input as DeleteInput } from 'mutations/deleteLinkMutation'
import upsertLinkMutation, { Input as UpsertInput } from 'mutations/upsertLinkMutation'
import updateLinkParentTopicsMutation, { Input as UpdateParentTopicsInput } from 'mutations/updateLinkParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'
import DeleteButton from 'components/ui/DeleteButton'
import { TopicOption } from 'components/types'
import { EditLinkForm_link as LinkType } from '__generated__/EditLinkForm_link.graphql'

type SelectedTopicsType = LinkType['selectedTopics']
type SelectedTopicType = NodeTypeOf<SelectedTopicsType>

type Props = {
  isOpen: boolean,
  link: LinkType,
  orgLogin: string,
  relay: RelayRefetchProp,
  toggleForm: () => void,
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
    const { prefix } = this.props.link.repository

    const input: UpsertInput = {
      // Keep the existing parent topics
      addParentTopicPath: null,
      repoPrefix: prefix,
      title: this.state.title,
      url: this.state.url,
    }

    upsertLinkMutation(this.props.relay.environment, input)
    this.props.toggleForm()
  }

  onDelete = () => {
    const input: DeleteInput = { linkPath: this.props.link.path }
    deleteLinkMutation(
      this.props.relay.environment,
      input,
      {
        configs: [{
          type: 'NODE_DELETE',
          deletedIDFieldName: 'deletedLinkPath',
        }],
      },
    )
  }

  get linkPath(): string {
    return this.props.link.path
  }

  get selectedTopics(): readonly SelectedTopicType[] {
    const { link } = this.props
    if (link.selectedTopics) {
      const array = liftNodes(link.selectedTopics) || []
      return makeOptions(array)
    }
    return []
  }

  updateTitle = (event: FormEvent<HTMLInputElement>) => {
    this.setState({ title: event.currentTarget.value })
  }

  updateTopics = (parentTopicPaths: string[]) => {
    const input: UpdateParentTopicsInput = {
      linkPath: this.linkPath,
      parentTopicPaths,
    }
    updateLinkParentTopicsMutation(this.props.relay.environment, input)
  }

  loadOptions = (searchString: string): Promise<readonly TopicOption[]> => {
    if (!this.props.relay) return new Promise(() => [])

    return new Promise((resolve) => {
      const variables = {
        orgLogin: this.props.orgLogin,
        count: 40,
        searchString,
      }

      this.props.relay.refetch(variables, null, () => {
        const { availableTopics } = this.props.link
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
        resolve(options)
      })
    })
  }

  updateUrl = (event: FormEvent<HTMLInputElement>) => {
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
            id={`edit-link-title-${this.linkPath}`}
            label="Page title"
            onChange={this.updateTitle}
            value={this.state.title}
          />
          <Input
            className="col-12"
            id={`edit-link-url-${this.linkPath}`}
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
            // @ts-ignore
            loadOptions={this.loadOptions}
            // @ts-ignore
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
      path
      title
      url

      repository {
        prefix
      }

      selectedTopics: parentTopics(first: 1000) {
        edges {
          node {
            value: path
            label: displayName
          }
        }
      }

      availableTopics: availableParentTopics(searchString: $searchString) {
        synonymMatches {
          value: path
          label: displayName
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
    $linkPath: String!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      link(path: $linkPath) {
        ...EditLinkForm_link @arguments(count: $count, searchString: $searchString)
      }
    }
  }
`)
