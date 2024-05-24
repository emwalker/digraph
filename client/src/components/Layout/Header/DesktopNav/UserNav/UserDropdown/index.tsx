import React, { useCallback } from 'react'
import { graphql, useFragment } from 'react-relay'
import { Link } from 'found'

import { UserDropdown_viewer$key } from '__generated__/UserDropdown_viewer.graphql'

type Props = {
  viewer: UserDropdown_viewer$key,
}

export default function UserDropdown(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment UserDropdown_viewer on User {
        name
        avatarUrl
      }
    `,
    props.viewer,
  )

  const signOut = useCallback(() => {
    window.location.href = '/logout'
  }, [])

  return (
    <div className="dropdown">
      <details className="dropdown details-reset details-overlay d-inline-block">
        <summary className="btn" aria-haspopup="true">
          <div className="summary">
            <img
              alt={viewer.name}
              className="avatar"
              height="20"
              src={`${viewer.avatarUrl}&s=40`}
              width="20"
            />
            <div className="dropdown-caret" />
          </div>
        </summary>

        <ul className="dropdown-menu dropdown-menu-sw">
          <li>
            <a className="dropdown-item" onClick={signOut} href="/logout">Sign out</a>
          </li>
          <li>
            <Link className="dropdown-item" to="/settings/account">Account</Link>
          </li>
          <li>
            <Link className="dropdown-item" to="/settings/support">Support</Link>
          </li>
        </ul>
      </details>
    </div>
  )
}
