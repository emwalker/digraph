import React, { Component, FormEvent } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { liftNodes, NodeTypeOf } from 'components/types'
import Input from 'components/ui/Input'
import deleteLinkMutation, { Input as DeleteInput } from 'mutations/deleteLinkMutation'
import upsertLinkMutation, { Input as UpsertInput } from 'mutations/upsertLinkMutation'
import updateLinkParentTopicsMutation, {
  Input as UpdateParentTopicsInput,
} from 'mutations/updateLinkParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import SaveOrCancel from 'components/ui/SaveOrCancel'
import DeleteButton from 'components/ui/DeleteButton'
import { TopicOption } from 'components/types'
import {
  EditLinkForm_linkDetails as LinkDetailsType,
} from '__generated__/EditLinkForm_linkDetails.graphql'
import { wikiRepoId } from 'components/constants'

type SelectedTopicsType = LinkDetailsType['selectedTopics']
type SelectedTopicType = NodeTypeOf<SelectedTopicsType>

type Props = {
  isOpen: boolean,
  linkDetails: LinkDetailsType,
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
      title: props.linkDetails.title,
      url: props.linkDetails.url,
    }
  }

  onSave = () => {
    const input: UpsertInput = {
      // Keep the existing parent topics
      addParentTopicId: null,
      // FIXME
      repoId: wikiRepoId,
      linkId: this.props.linkDetails.linkId,
      title: this.state.title,
      url: this.state.url,
    }

    upsertLinkMutation(this.props.relay.environment, input)
    this.props.toggleForm()
  }

  onDelete = () => {
    // FIXME: use selected repo
    const input: DeleteInput = { linkId: this.props.linkDetails.linkId, repoId: wikiRepoId }
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
    return this.props.linkDetails.linkId
  }

  get selectedTopics(): readonly SelectedTopicType[] {
    const { linkDetails: link } = this.props
    if (link.selectedTopics) {
      const array = liftNodes(link.selectedTopics) || []
      return makeOptions(array)
    }
    return []
  }

  updateTitle = (event: FormEvent<HTMLInputElement>) => {
    this.setState({ title: event.currentTarget.value })
  }

  updateTopics = (parentTopicIds: string[]) => {
    const input: UpdateParentTopicsInput = {
      linkId: this.linkId,
      parentTopicIds,
      // FIXME
      repoId: wikiRepoId,
    }
    updateLinkParentTopicsMutation(this.props.relay.environment, input)
  }

  loadOptions = (searchString: string): Promise<readonly TopicOption[]> => {
    if (!this.props.relay) return new Promise(() => [])

    return new Promise((resolve) => {
      const variables = {
        count: 40,
        searchString,
      }

      this.props.relay.refetch(variables, null, () => {
        const { availableTopics } = this.props.linkDetails
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
  linkDetails: graphql`
    fragment EditLinkForm_linkDetails on LinkDetails @argumentDefinitions(
      searchString: {type: "String", defaultValue: null},
      count: {type: "Int!", defaultValue: 10}
    ) {
      linkId
      title
      url

      selectedTopics: parentTopics(first: 1000) {
        edges {
          node {
            value: id
            label: displayName
          }
        }
      }

      availableTopics: availableParentTopics(searchString: $searchString) {
        synonymMatches {
          value: id
          label: displayName
        }
      }
    }
  `,
},
graphql`
  query EditLinkFormRefetchQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $linkId: String!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      repositoryIds: $repoIds,
    ) {
      link(id: $linkId) {
        details {
          ...EditLinkForm_linkDetails @arguments(count: $count, searchString: $searchString)
        }
      }
    }
  }
`)
