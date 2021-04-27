import React, { Component } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { TopicOption } from 'components/types'
import deleteTopicMutation, { Input as DeleteInput } from 'mutations/deleteTopicMutation'
import updateTopicMutation, { Input as UpdateInput } from 'mutations/updateTopicMutation'
import updateTopicTopicsMutation, {
  Input as UpdateTopicsInput,
} from 'mutations/updateTopicParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { EditTopicForm_topic as TopicType } from '__generated__/EditTopicForm_topic.graphql'
import Synonyms from './Synonyms'
import TopicTimeRange from './TopicTimeRange'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  relay: RelayRefetchProp,
  toggleForm: () => void,
  topic: TopicType,
}

type State = {
  description: string | null,
  displayName: string,
}

class EditTopicForm extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      displayName: props.topic.displayName,
      description: props.topic.description,
    }
  }

  onSave = () => {
    const input: UpdateInput = {
      topicIds: this.addTopicIds,
      description: this.state.description || '',
      id: this.props.topic.id,
      name: this.state.displayName,
    }
    updateTopicMutation(this.props.relay.environment, input)
    this.props.toggleForm()
  }

  onDelete = () => {
    const input: DeleteInput = { topicId: this.props.topic.id }
    deleteTopicMutation(
      this.props.relay.environment,
      input,
      {
        configs: [{
          type: 'NODE_DELETE',
          deletedIDFieldName: 'deletedTopicId',
        }],
      },
    )
  }

  // eslint-disable-next-line class-methods-use-this
  get addTopicIds(): string[] {
    return []
  }

  get topicId(): string {
    return this.props.topic.id
  }

  get selectedTopics(): TopicOption[] | null {
    const { selectedTopics } = this.props.topic
    return selectedTopics ? makeOptions(selectedTopics) : null
  }

  updateParentTopics = (parentTopicIds: string[]) => {
    const input: UpdateTopicsInput = {
      topicId: this.props.topic.id,
      parentTopicIds,
    }
    updateTopicTopicsMutation(this.props.relay.environment, input)
  }

  loadOptions = (searchString: string): Promise<TopicOption[]> => {
    if (!this.props.relay) return new Promise(() => [])

    return new Promise((resolve) => {
      const variables = {
        orgLogin: this.props.orgLogin,
        count: 60,
        searchString,
      }

      this.props.relay.refetch(variables, null, () => {
        const { availableTopics } = this.props.topic
        const options = availableTopics ? makeOptions(availableTopics) : []
        resolve(options)
      })
    })
  }

  render = () => {
    if (!this.props.isOpen) return null

    const { selectedTopics } = this

    return (
      selectedTopics ? (
        <div className="my-4">
          <Synonyms topic={this.props.topic} />
          <TopicTimeRange topic={this.props.topic} />

          <EditTopicList
            loadOptions={this.loadOptions}
            selectedTopics={selectedTopics}
            updateTopics={this.updateParentTopics}
          />

          <dl className="form-group">
            <DeleteButton
              onDelete={this.onDelete}
            />
            <button
              className="btn-link float-right"
              onClick={this.props.toggleForm}
              type="button"
            >
              Close
            </button>
          </dl>
        </div>
      ) : null
    )
  }
}

export default createRefetchContainer(EditTopicForm, {
  topic: graphql`
    fragment EditTopicForm_topic on Topic @argumentDefinitions(
      searchString: {type: "String", defaultValue: null},
      count: {type: "Int!", defaultValue: 10}
    ) {
      description
      id
      displayName: name

      selectedTopics: parentTopics(first: 1000) {
        edges {
          node {
            value: id
            label: name
          }
        }
      }

      availableTopics: availableParentTopics(first: $count, searchString: $searchString) {
        edges {
          node {
            value: id
            label: name
          }
        }
      }

      ...Synonyms_topic
      ...TopicTimeRange_topic
    }
  `,
},
graphql`
  query EditTopicFormRefetchQuery(
    $viewerId: ID!,
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
    $topicId: ID!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      topic(id: $topicId) {
        ...EditTopicForm_topic @arguments(count: $count, searchString: $searchString)
      }
    }
  }
`)
