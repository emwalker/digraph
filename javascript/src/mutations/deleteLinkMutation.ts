import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'
import { deleteLinkMutation } from '__generated__/deleteLinkMutation.graphql'

const query = graphql`
  mutation deleteLinkMutation(
    $input: DeleteLinkInput!
  ) {
    deleteLink(input: $input) {
      clientMutationId
      deletedLinkId
    }
  }
`

export function makeDeleteLinkCallback({ selectedRepoId, linkId }: {
  linkId: string,
  selectedRepoId: string | null,
}) {
  const deleteLink = useMutation<deleteLinkMutation>(query)[0]

  return useCallback(() => {
    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

    deleteLink({
      variables: {
        input: { repoId: selectedRepoId, linkId },
      },
    })
  }, [deleteLink, selectedRepoId, linkId])
}
