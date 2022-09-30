import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'

import {
  updateLinkParentTopicsMutation,
} from '__generated__/updateLinkParentTopicsMutation.graphql'

const query = graphql`
  mutation updateLinkParentTopicsMutation(
    $input: UpdateLinkParentTopicsInput!
  ) {
    updateLinkParentTopics(input: $input) {
      link {
        repoLinks {
          ...EditRepoLink_repoLink
        }
        ...Link_link
      }
    }
  }
`

type Props = {
  linkId: string,
  selectedRepoId: string,
}

export function makeUpdateLinkParentTopicsCallback({ linkId, selectedRepoId }: Props) {
  const updateParentTopics = useMutation<updateLinkParentTopicsMutation>(query)[0]

  return useCallback((parentTopicIds: string[]) => {
    updateParentTopics({
      variables: {
        input: {
          linkId,
          parentTopicIds,
          repoId: selectedRepoId,
        },
      },
    })
  }, [updateParentTopics, linkId, selectedRepoId])
}
