import React, { Component } from 'react'
import { createRefetchContainer, graphql, RelayRefetchProp } from 'react-relay'

import { TopicOption, liftNodes } from 'components/types'
import deleteTopicMutation, { Input as DeleteInput } from 'mutations/deleteTopicMutation'
import updateTopicTopicsMutation, {
  Input as UpdateTopicsInput,
} from 'mutations/updateTopicParentTopicsMutation'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import {
  EditTopicForm_topic as TopicType,
} from '__generated__/EditTopicForm_topic.graphql'
import Synonyms from './Synonyms'
import TopicTimerange from './TopicTimerange'
import { wikiRepoId } from 'components/constants'

type TopicDetailType = TopicType['details'][0]

type Props = {
  isOpen: boolean,
  relay: RelayRefetchProp,
  toggleForm: () => void,
  topic: TopicType,
}

type State = {
  displayName: string,
}

class EditTopicForm extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      displayName: props.topic.displayName,
    }
  }

  onDelete = () => {
    const topicId = this.topicDetail?.topicId

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
  get topicDetail(): TopicDetailType | null {
    const details = this.props.topic.details
    if (details.length < 1) return null
    return details[0]
  }

  get selectedTopics(): TopicOption[] | null {
    const selectedTopics = this.topicDetail?.selectedTopics
    const array = liftNodes(selectedTopics)
    return selectedTopics ? makeOptions(array) : null
  }

  updateParentTopics = (parentTopicIds: string[]) => {
    const topicId = this.topicDetail?.topicId
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
        const availableTopics = this.topicDetail?.availableTopics
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
        resolve(options as TopicOption[])
      })
    })
  }

  render = () => {
    if (!this.props.isOpen) return null

    const { selectedTopics } = this
    const topicDetail = this.topicDetail

    if (!topicDetail) return null

    return (
      selectedTopics ? (
        <div className="my-4">
          <Synonyms topic={this.props.topic} />
          <TopicTimerange topicDetail={topicDetail} />

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
      displayName
      ...Synonyms_topic

      details {
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

        ...TopicTimerange_topicDetail
      }
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
