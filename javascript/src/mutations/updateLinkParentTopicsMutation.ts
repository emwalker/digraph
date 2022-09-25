import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'

import { updateLinkParentTopicsMutation } from '__generated__/updateLinkParentTopicsMutation.graphql'

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

export function makeUpdateLinkParentTopicsCallback({ linkId, selectedRepoId }: {
  linkId: string,
  selectedRepoId: string | null,
}) {
  const updateParentTopics = useMutation<updateLinkParentTopicsMutation>(query)[0]

  return useCallback((parentTopicIds: string[]) => {
    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

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
