import React, { useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'

import { UserDropdown_viewer as Viewer } from '__generated__/UserDropdown_viewer.graphql'

type Props = {
  viewer: Viewer,
}

const UserDropdown = ({ viewer: { name, avatarUrl } }: Props) => {
  const signOut = useCallback(() => {
    window.location.href = '/logout'
  }, [])

  return (
    <div className="dropdown">
      <details className="dropdown details-reset details-overlay d-inline-block">
        <summary className="btn" aria-haspopup="true">
          <div className="summary">
            <img
              alt={name}
              className="avatar"
              height="20"
              src={`${avatarUrl}&s=40`}
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

export default createFragmentContainer(UserDropdown, {
  viewer: graphql`
    fragment UserDropdown_viewer on User {
      name
      avatarUrl
    }
  `,
})
