import React, { useCallback } from 'react'
import { fetchQuery, graphql, useFragment, useRelayEnvironment } from 'react-relay'

import { TopicOption, liftNodes } from 'components/types'
import { makeUpdateTopicParentTopicsCallback } from 'mutations/updateTopicParentTopicsMutation'
import EditParentTopicList, { makeOptions } from 'components/ui/EditParentTopicList'
import RefetchQuery from '../RepoTopicParentTopicsRefetchQuery'
import {
  RepoTopicParentTopicsRefetchQuery,
} from '__generated__/RepoTopicParentTopicsRefetchQuery.graphql'
import {
  RepoTopicParentTopics_repoTopic$key,
} from '__generated__/RepoTopicParentTopics_repoTopic.graphql'

type LoadOptionsProps = {
  selectedRepoId: string,
  topicId: string,
  viewerId: string,
}

function makeLoadOptions({ selectedRepoId, topicId, viewerId }: LoadOptionsProps) {
  const environment = useRelayEnvironment()

  return useCallback((searchString: string) => {
    return new Promise<readonly TopicOption[]>((resolve) => {
      const variables = { selectedRepoId, topicId, viewerId, searchString }
      fetchQuery<RepoTopicParentTopicsRefetchQuery>(environment, RefetchQuery, variables)
        .subscribe({
          next(data) {
            const options =
              makeOptions(data.view.topic?.repoTopic?.availableParentTopics.synonymMatches || [])
            resolve(options)
          },
        })
    })
  }, [selectedRepoId, viewerId, topicId])
}

const repoTopicFragment = graphql`
  fragment RepoTopicParentTopics_repoTopic on RepoTopic {
    topicId

    selectedTopics: parentTopics(first: 1000) {
      edges {
        node {
          value: id
          label: displayName
        }
      }
    }
  }
`

type Props = {
  repoTopic: RepoTopicParentTopics_repoTopic$key,
  selectedRepoId: string,
  viewerId: string,
}

export default function RepoTopicParentTopics({ selectedRepoId, viewerId, ...rest }: Props) {
  const repoTopic = useFragment(repoTopicFragment, rest.repoTopic)
  const topicId = repoTopic.topicId
  const selectedTopics = makeOptions(liftNodes(repoTopic.selectedTopics))
  const loadOptions = makeLoadOptions({ selectedRepoId, topicId, viewerId })
  const updateTopics = makeUpdateTopicParentTopicsCallback({ repoId: selectedRepoId, topicId })

  return (
    <EditParentTopicList
      loadOptions={loadOptions}
      selectedTopics={selectedTopics}
      updateTopics={updateTopics}
    />
  )
}