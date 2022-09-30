import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'

import {
  updateTopicParentTopicsMutation,
} from '__generated__/updateTopicParentTopicsMutation.graphql'

const query = graphql`
  mutation updateTopicParentTopicsMutation(
    $input: UpdateTopicParentTopicsInput!
  ) {
    updateTopicParentTopics(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic
      }
    }
  }
`

type Props = {
  repoId: string | null,
  topicId: string,
}

export function makeUpdateTopicParentTopicsCallback({ repoId, topicId }: Props) {
  const updateTopics = useMutation<updateTopicParentTopicsMutation>(query)[0]

  return useCallback((parentTopicIds: string[]) => {
    if (!repoId) {
      console.log('no topic')
      return
    }

    updateTopics({
      variables: {
        input: {
          repoId,
          topicId,
          parentTopicIds,
        },
      },
    })
  }, [query])
}
