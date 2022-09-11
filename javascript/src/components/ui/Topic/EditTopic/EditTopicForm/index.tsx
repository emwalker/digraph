import React, { Component } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { TopicOption, liftNodes } from 'components/types'
import deleteTopicMutation, { Input as DeleteInput } from 'mutations/deleteTopicMutation'
import updateTopicTopicsMutation, {
  Input as UpdateTopicsInput,
} from 'mutations/updateTopicParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { EditTopicForm_topic$data as TopicType } from '__generated__/EditTopicForm_topic.graphql'
import Synonyms from './Synonyms'
import TopicTimerange from './TopicTimerange'
import { wikiRepoId } from 'components/constants'

type RepoTopicType = TopicType['repoTopics'][0]

type Props = {
  isOpen: boolean,
  relay: RelayRefetchProp,
  toggleForm: () => void,
  topic: TopicType,
}

type State = {
  displayName: string | undefined,
}

class EditTopicForm extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      displayName: props.topic.displayName,
    }
  }

  onDelete = () => {
    const topicId = this.repoTopic?.topicId

    if (!topicId) return

    // FIXME: use selected repo
    const input: DeleteInput = { repoId: wikiRepoId, topicId }
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

  // FIXME
  get repoTopic(): RepoTopicType | null {
    const repoTopics = this.props.topic?.repoTopics
    if (repoTopics.length < 1) return null
    return repoTopics[0]
  }

  get selectedTopics(): TopicOption[] | null {
    const selectedTopics = this.repoTopic?.selectedTopics
    const array = liftNodes(selectedTopics)
    return selectedTopics ? makeOptions(array) : null
  }

  updateParentTopics = (parentTopicIds: string[]) => {
    const topicId = this.repoTopic?.topicId
    if (!topicId) return

    const input: UpdateTopicsInput = {
      // FIXME: use id instead of prefix
      repoId: wikiRepoId,
      topicId,
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
        const availableTopics = this.repoTopic?.availableTopics
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
        resolve(options as TopicOption[])
      })
    })
  }

  render = () => {
    if (!this.props.isOpen) return null

    const { selectedTopics } = this
    const repoTopic = this.repoTopic

    if (!repoTopic) return null
    if (!selectedTopics) return null

    return (
      <div className="my-4">
        <Synonyms topic={this.props.topic} />
        <TopicTimerange repoTopic={repoTopic} />

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
    )
  }
}

export default createRefetchContainer(EditTopicForm, {
  topic: graphql`
    fragment EditTopicForm_topic on Topic @argumentDefinitions(
      searchString: {type: "String", defaultValue: null},
    ) {
      displayName
      ...Synonyms_topic

      repoTopics {
        topicId

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

        ...TopicTimerange_repoTopic
      }
    }
  `,
},
graphql`
  query EditTopicFormRefetchQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: String!,
    $searchString: String,
  ) {
    view(
      viewerId: $viewerId,
      repositoryIds: $repoIds,
    ) {
      topic(id: $topicId) {
        ...EditTopicForm_topic @arguments(searchString: $searchString)
      }
    }
  }
`)
