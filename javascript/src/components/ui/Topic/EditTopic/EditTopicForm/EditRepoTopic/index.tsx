import React, { useCallback } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import Synonyms from './Synonyms'
import RepoTopicTimerange from './RepoTopicTimerange'
import EditTopicList, { makeOptions } from 'components/ui/EditTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { TopicOption, liftNodes } from 'components/types'
import deleteTopicMutation, { Input as DeleteInput } from 'mutations/deleteTopicMutation'
import updateTopicTopicsMutation, {
  Input as UpdateTopicsInput,
} from 'mutations/updateTopicParentTopicsMutation'
import { EditRepoTopic_repoTopic$key } from '__generated__/EditRepoTopic_repoTopic.graphql'
import { EditRepoTopic_viewer$key } from '__generated__/EditRepoTopic_viewer.graphql'

type Props = {
  repoTopic: EditRepoTopic_repoTopic$key,
  viewer: EditRepoTopic_viewer$key,
}

// const refetchQuery = graphql`
//   query EditTopicFormRefetchQuery(
//     $viewerId: ID!,
//     $repoIds: [ID!],
//     $topicId: String!,
//     $searchString: String,
//   ) {
//     view(
//       viewerId: $viewerId,
//       repositoryIds: $repoIds,
//     ) {
//       viewer {
//         ...EditTopicForm_viewer
//       }

//       topic(id: $topicId) {
//         ...EditTopicForm_topic
//       }
//     }
//   }
// `

const repoTopicFragment = graphql`
  fragment EditRepoTopic_repoTopic on RepoTopic @argumentDefinitions(
    searchString: {type: "String", defaultValue: null},
  ) {
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

    ...Synonyms_repoTopic
    ...RepoTopicTimerange_repoTopic
  }
`

const viewerFragment = graphql`
  fragment EditRepoTopic_viewer on User {
    selectedRepository {
      id
    }

    ...RepoTopicTimerange_viewer
    ...Synonyms_viewer
  }
`

export default function EditRepoTopic(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const environment = useRelayEnvironment()

  const selectedRepoId = viewer.selectedRepository?.id

  const loadOptions = useCallback((searchString: string): Promise<readonly TopicOption[]> => {
    return new Promise((resolve) => {
      const variables = {
        count: 60,
        searchString,
      }
  
      props.relay.refetch(variables, null, () => {
        const availableTopics = repoTopic?.availableTopics
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
        resolve(options as TopicOption[])
      })
    })
  }, [repoTopic])

  const onDelete = useCallback(() => {
    const topicId = repoTopic?.topicId
    if (!topicId) return
  
    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }
  
    const input: DeleteInput = { repoId: selectedRepoId, topicId }
    deleteTopicMutation(
      environment,
      input,
      {
        configs: [{
          type: 'NODE_DELETE',
          deletedIDFieldName: 'deletedTopicId',
        }],
      },
    )
  }, [environment, deleteTopicMutation, repoTopic])

  const updateParentTopics = useCallback((parentTopicIds: string[]) => {
      const topicId = repoTopic?.topicId
      if (!topicId) return
    
      if (!selectedRepoId) {
        console.log('no repo selected')
        return
      }
    
      const input: UpdateTopicsInput = {
        repoId: selectedRepoId,
        topicId,
        parentTopicIds,
      }
      updateTopicTopicsMutation(environment, input)
    }, [repoTopic, selectedRepoId, environment, updateTopicTopicsMutation]
  )

  const topics = repoTopic.selectedTopics
  const selectedTopics = topics ?  makeOptions(liftNodes(topics)) : []

  return (
    <div className="my-4">
      <Synonyms viewer={viewer} repoTopic={repoTopic} />
      <RepoTopicTimerange viewer={viewer} repoTopic={repoTopic} />

      <EditTopicList
        loadOptions={loadOptions}
        selectedTopics={selectedTopics}
        updateTopics={updateParentTopics}
      />

      <DeleteButton onDelete={onDelete} />
    </div>
  )
}
