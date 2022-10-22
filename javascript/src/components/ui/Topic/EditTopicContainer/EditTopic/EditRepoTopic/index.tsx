import React, { useCallback } from 'react'
import { Disposable, graphql, useFragment, useMutation, UseMutationConfig } from 'react-relay'

import RepoTopicSynonyms from './RepoTopicSynonyms'
import RepoTopicTimerange from './RepoTopicTimerange'
import DeleteButton from 'components/ui/DeleteButton'
import deleteTopicQuery from 'mutations/deleteTopicMutation'
import { deleteTopicMutation } from '__generated__/deleteTopicMutation.graphql'
import {
  EditRepoTopic_repoTopic$key,
  EditRepoTopic_repoTopic$data as RepoTopicType,
} from '__generated__/EditRepoTopic_repoTopic.graphql'
import { EditRepoTopic_viewer$key } from '__generated__/EditRepoTopic_viewer.graphql'
import ParentTopics from './RepoTopicParentTopics'
import { borderColor } from 'components/helpers'

type Props = {
  repoTopic: EditRepoTopic_repoTopic$key,
  viewer: EditRepoTopic_viewer$key,
}

function makeOnDelete({ deleteTopic, selectedRepoId, repoTopic }: {
  deleteTopic: (config: UseMutationConfig<deleteTopicMutation>) => Disposable,
  selectedRepoId: string | null,
  repoTopic: RepoTopicType,
}) {
  return useCallback(() => {
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
}

const repoTopicFragment = graphql`
  fragment EditRepoTopic_repoTopic on RepoTopic {
    topicId
    displayColor

    ...RepoTopicSynonyms_repoTopic
    ...RepoTopicTimerange_repoTopic
    ...RepoTopicParentTopics_repoTopic
  }
`

const viewerFragment = graphql`
  fragment EditRepoTopic_viewer on User {
    id
    selectedRepoId

    ...RepoTopicTimerange_viewer
    ...RepoTopicTimerangeForm_viewer
    ...RepoTopicSynonyms_viewer
  }
`

export default function EditRepoTopic(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const deleteTopic = useMutation<deleteTopicMutation>(deleteTopicQuery)[0]

  const viewerId = viewer?.id || null
  const selectedRepoId = viewer.selectedRepoId || null
  const onDelete = makeOnDelete({ deleteTopic, repoTopic, selectedRepoId })

  if (!selectedRepoId) {
    console.log('no repo selected')
    return null
  }

  if (!viewerId) {
    console.log('no viewer')
    return null
  }

  return (
    <li
      className="Box-row edit-repo-topic"
      style={{ borderColor: borderColor(repoTopic.displayColor) }}
    >
      <RepoTopicSynonyms viewer={viewer} repoTopic={repoTopic} />
      <RepoTopicTimerange viewer={viewer} repoTopic={repoTopic} />

      <ParentTopics
        selectedRepoId={selectedRepoId}
        repoTopic={repoTopic}
        viewerId={viewerId}
      />

      <DeleteButton onDelete={onDelete} />
    </li>
  )
}
