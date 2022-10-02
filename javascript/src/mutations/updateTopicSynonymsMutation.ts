import { SynonymType } from 'components/types'
import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'
import { updateTopicSynonymsMutation } from '__generated__/updateTopicSynonymsMutation.graphql'

import {
  RepoTopicSynonyms_repoTopic$data as RepoTopicType,
} from '__generated__/RepoTopicSynonyms_repoTopic.graphql'


function name(synonyms: readonly SynonymType[]) {
  if (synonyms.length > 0) {
    for (const synonym of synonyms) {
      if (synonym.locale !== 'en') // FIXME
        continue
      return synonym.name
    }
    return synonyms[0].name
  }

  return 'Missing name'
}

function displayName(synonyms: readonly SynonymType[], timerangPrefix: string | null) {
  const suffix = name(synonyms)
  return timerangPrefix ? `${timerangPrefix} ${suffix}` : suffix
}

function optimisticResponse(repoTopic: RepoTopicType, synonymUpdate: SynonymType[]) {
  return {
    updateTopicSynonyms: {
      clientMutationId: null,
      alerts: [],
      updatedTopic: {
        id: repoTopic.topicId,
        displayName: displayName(repoTopic.synonyms, repoTopic.timerangePrefix),
      },
      updatedRepoTopic: {
        ...repoTopic,
        synonyms: synonymUpdate,
      },
    },
  }
}

const query = graphql`
  mutation updateTopicSynonymsMutation($input: UpdateTopicSynonymsInput!) {
    updateTopicSynonyms(input: $input) {
      clientMutationId

      alerts {
        id
        text
        type
      }

      updatedTopic {
        id
        displayName
      }

      updatedRepoTopic {
        id
        ...RepoTopicSynonyms_repoTopic
      }
    }
  }
`

type Props = {
  selectedRepoId: string | null,
  repoTopic: RepoTopicType,
  setInputName: (inputName: string) => void,
}

export function makeUpdateTopicSynonymsCallback({
  selectedRepoId, repoTopic, setInputName,
}: Props) {
  const updateSynonyms = useMutation<updateTopicSynonymsMutation>(query)[0]

  return useCallback((synonymUpdate: SynonymType[]) => {
    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

    updateSynonyms({
      variables: {
        input: { repoId: selectedRepoId, topicId: repoTopic.topicId, synonyms: synonymUpdate },
      },
      optimisticResponse: optimisticResponse(repoTopic, synonymUpdate),
    })

    setInputName('')
  }, [selectedRepoId, repoTopic, setInputName, updateSynonyms])
}
