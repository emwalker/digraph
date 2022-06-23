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
      addParentTopicPaths: this.addParentTopicPaths,
      description: this.state.description || '',
      topicPath: this.props.topic.path,
      name: this.state.displayName,
    }
    updateTopicMutation(this.props.relay.environment, input)
    this.props.toggleForm()
  }

  onDelete = () => {
    const input: DeleteInput = { topicPath: this.props.topic.path }
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
  get addParentTopicPaths(): string[] {
    return []
  }

  get topicPath(): string {
    return this.props.topic.path
  }

  get selectedTopics(): TopicOption[] | null {
    const { selectedTopics } = this.props.topic
    // @ts-ignore
    return selectedTopics ? makeOptions(selectedTopics) : null
  }

  updateParentTopics = (parentTopicPaths: string[]) => {
    const input: UpdateTopicsInput = {
      topicPath: this.props.topic.path,
      parentTopicPaths,
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
        resolve(options as TopicOption[])
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
      path

      selectedTopics: parentTopics(first: 1000) {
        edges {
          node {
            value: path
            label: name
          }
        }
      }

      availableTopics: availableParentTopics(first: $count, searchString: $searchString) {
        edges {
          node {
            value: path
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
    $topicPath: String!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      topic(path: $topicPath) {
        ...EditTopicForm_topic @arguments(count: $count, searchString: $searchString)
      }
    }
  }
`)
