import React, { useCallback } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'

import RepoTopicSynonyms from './RepoTopicSynonyms'
import RepoTopicTimerange from './RepoTopicTimerange'
import EditParentTopicList, { makeOptions } from 'components/ui/EditParentTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { TopicOption, liftNodes } from 'components/types'
import deleteTopicQuery from 'mutations/deleteTopicMutation'
import { deleteTopicMutation } from '__generated__/deleteTopicMutation.graphql'
import updateParentTopicsQuery from 'mutations/updateTopicParentTopicsMutation'
import { updateTopicParentTopicsMutation } from '__generated__/updateTopicParentTopicsMutation.graphql'
import { EditRepoTopic_repoTopic$key } from '__generated__/EditRepoTopic_repoTopic.graphql'
import { EditRepoTopic_viewer$key } from '__generated__/EditRepoTopic_viewer.graphql'
import { borderColor } from 'components/helpers'

type Props = {
  repoTopic: EditRepoTopic_repoTopic$key,
  viewer: EditRepoTopic_viewer$key,
}

const repoTopicFragment = graphql`
  fragment EditRepoTopic_repoTopic on RepoTopic @argumentDefinitions(
    searchString: {type: "String", defaultValue: null},
  ) {
    topicId
    displayColor

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

    ...RepoTopicSynonyms_repoTopic
    ...RepoTopicTimerange_repoTopic
  }
`

const viewerFragment = graphql`
  fragment EditRepoTopic_viewer on User {
    selectedRepository {
      id
    }

    ...RepoTopicTimerange_viewer
    ...RepoTopicTimerangeForm_viewer
    ...RepoTopicSynonyms_viewer
  }
`

export default function EditRepoTopic(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const deleteTopic = useMutation<deleteTopicMutation>(deleteTopicQuery)[0]
  const updateParentTopics
    = useMutation<updateTopicParentTopicsMutation>(updateParentTopicsQuery)[0]

  const selectedRepoId = viewer.selectedRepository?.id

  const loadOptions = useCallback((/* searchString: string */): Promise<readonly TopicOption[]> => {
    return new Promise((resolve) => {
      // const variables = {
      //   count: 60,
      //   searchString,
      // }

      resolve([] as TopicOption[])
    })
  }, [repoTopic])

  const onDelete = useCallback(() => {
    const topicId = repoTopic?.topicId
    if (!topicId) return

    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

    deleteTopic({
      variables: {
        input: { repoId: selectedRepoId, topicId },
      },
      configs: [{
        type: 'NODE_DELETE',
        deletedIDFieldName: 'deletedTopicId',
      }],
    })
  }, [deleteTopic, repoTopic])

  const updateTopics = useCallback((parentTopicIds: string[]) => {
    const topicId = repoTopic?.topicId
    if (!topicId) return

    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

    updateParentTopics({
      variables: {
        input: {
          repoId: selectedRepoId,
          topicId,
          parentTopicIds,
        },
      },
    })
  }, [repoTopic, selectedRepoId, updateParentTopics])

  const topics = repoTopic.selectedTopics
  const selectedTopics = topics ? makeOptions(liftNodes(topics)) : []

  return (
    <li className="Box-row" style={{ borderColor: borderColor(repoTopic.displayColor) }}>
      <RepoTopicSynonyms viewer={viewer} repoTopic={repoTopic} />
      <RepoTopicTimerange viewer={viewer} repoTopic={repoTopic} />

      <EditParentTopicList
        loadOptions={loadOptions}
        selectedTopics={selectedTopics}
        updateTopics={updateTopics}
      />

      <DeleteButton onDelete={onDelete} />
    </li>
  )
}
