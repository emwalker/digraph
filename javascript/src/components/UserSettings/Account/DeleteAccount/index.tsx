import React, { useCallback } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'

import deleteAccountQuery from 'mutations/deleteAccountMutation'
import { deleteAccountMutation } from '__generated__/deleteAccountMutation.graphql'
import { DeleteAccount_view$key } from '__generated__/DeleteAccount_view.graphql'

declare let confirm: Function

type Props = {
  view: DeleteAccount_view$key,
}

const viewFragment = graphql`
  fragment DeleteAccount_view on View {
    viewer {
      id
    }
  }
`

export default function DeleteAccount(props: Props) {
  const view = useFragment(viewFragment, props.view)
  const [deleteAccount, deleteAccountInFlight] =
    useMutation<deleteAccountMutation>(deleteAccountQuery)
  const viewer = view?.viewer

  const onClick = useCallback(async () => {
    if (!viewer) {
      console.log('There is no viewer')
      return
    }

    if (!confirm('Are you sure you want to delete your account?')) return

    const { id: userId } = viewer
    if (!userId) return

    deleteAccount({ variables: { input: { userId } } })

    setTimeout(
      () => {
        document.location.replace('/logout')
      },
      5000,
    )
  }, [deleteAccount, viewer, setTimeout])

  return (
    <>
      <div className="Subhead">
        <div className="Subhead-heading Subhead-heading--danger">Delete account</div>
      </div>

      <p>
        Your user information and private repo will be permanently removed when you delete your
        account. Links and topics that you have added to the public collection will still be there,
        but your account will no longer be associated with them.
      </p>

      <button
        className="btn btn-danger"
        disabled={deleteAccountInFlight}
        onClick={onClick}
        type="button"
      >
        Delete your account
      </button>

      <p className="mt-5">
        To revoke GitHub auth permission for this account, go to the
        {' '}
        <a href="https://github.com/settings/applications">
          Authorized OAuth Apps
        </a>
        {' '}
        menu and look for &quot;Digraph&quot;.
        If you run into any difficulties deleting your account, email the
        {' '}
        <a href="mailto:eric.walker@gmail.com?subject=Problem deleting account">app maintainer</a>
        {' '}
        with a description of the problem.
      </p>
    </>
  )
}
