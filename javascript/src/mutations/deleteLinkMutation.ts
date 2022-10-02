import { useCallback } from 'react'
import { graphql, useMutation } from 'react-relay'
import { RecordSourceSelectorProxy } from 'relay-runtime'
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

  const updater = (store: RecordSourceSelectorProxy) => {
    store.delete(linkId)
  }

  return useCallback(() => {
    if (!selectedRepoId) {
      console.log('no repo selected')
      return
    }

    deleteLink({
      variables: {
        input: { repoId: selectedRepoId, linkId },
      },
      optimisticUpdater: updater,
      updater,
    })
  }, [deleteLink, selectedRepoId, linkId, updater])
}
