import React, { useState, useCallback } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import deleteAccountMutation, { Input } from 'mutations/deleteAccountMutation'
import { DeleteAccount_view$key } from '__generated__/DeleteAccount_view.graphql'

declare let confirm: Function

type Props = {
  view: DeleteAccount_view$key,
}

export default function DeleteAccount(props: Props) {
  const view = useFragment(
    graphql`
      fragment DeleteAccount_view on View {
        viewer {
          id
        }
      }
    `,
    props.view,
  )

  const [mutationInFlight, setMutationInFlight] = useState(false)
  const viewer = view?.viewer

  const onClick = useCallback(async () => {
    if (!viewer) {
      // eslint-disable-next-line no-console
      console.log('There is no viewer')
      return
    }

    // eslint-disable-next-line no-restricted-globals
    if (!confirm('Are you sure you want to delete your account?')) return

    const { id: userId } = viewer
    if (!userId) return

    setMutationInFlight(true)
    const input: Input = { userId }
    deleteAccountMutation(useRelayEnvironment(), input)

    setTimeout(
      () => {
        document.location.replace('/logout')
      },
      5000,
    )
  }, [mutationInFlight, viewer])

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
        disabled={mutationInFlight}
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
