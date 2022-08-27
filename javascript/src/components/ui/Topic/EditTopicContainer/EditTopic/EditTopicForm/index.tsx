import React, { Component } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { TopicOption, liftNodes } from 'components/types'
import deleteTopicMutation, { Input as DeleteInput } from 'mutations/deleteTopicMutation'
import updateTopicTopicsMutation, {
  Input as UpdateTopicsInput,
} from 'mutations/updateTopicParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { EditTopicForm_topic as TopicType } from '__generated__/EditTopicForm_topic.graphql'
import Synonyms from './Synonyms'
import TopicTimerange from './TopicTimerange'

type Props = {
  isOpen: boolean,
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

  onDelete = () => {
    // FIXME: use selected repo
    const repoId = '/wiki/'

    const input: DeleteInput = { repoId, topicId: this.props.topic.id }
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

  get topicId(): string {
    return this.props.topic.id
  }

  get selectedTopics(): TopicOption[] | null {
    const { selectedTopics } = this.props.topic
    const array = liftNodes(selectedTopics)
    return selectedTopics ? makeOptions(array) : null
  }

  updateParentTopics = (parentTopicIds: string[]) => {
    const input: UpdateTopicsInput = {
      // FIXME: use id instead of prefix
      repoId: '/wiki/',
      topicId: this.props.topic.id,
      parentTopicIds,
    }
    updateTopicTopicsMutation(this.props.relay.environment, input)
  }

  loadOptions = (searchString: string): Promise<TopicOption[]> => {
    if (!this.props.relay) return new Promise(() => [])

    return new Promise((resolve) => {
      const variables = {
        count: 60,
        searchString,
      }

      this.props.relay.refetch(variables, null, () => {
        const { availableTopics } = this.props.topic
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
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
          <TopicTimerange topic={this.props.topic} />

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

      availableTopics: availableParentTopics(searchString: $searchString) {
        synonymMatches {
          value: id
          label: displayName
        }
      }

      ...Synonyms_topic
      ...TopicTimerange_topic
    }
  `,
},
graphql`
  query EditTopicFormRefetchQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: String!,
    $count: Int!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      repositoryIds: $repoIds,
    ) {
      topic(id: $topicId) {
        ...EditTopicForm_topic @arguments(count: $count, searchString: $searchString)
      }
    }
  }
`)
