// @flow
import React, { useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Avatar } from '@primer/components'
import { Link } from 'found'

import type { UserDropdown_viewer as Viewer } from './__generated__/UserDropdown_viewer.graphql'
import styles from './styles.module.css'

type Props = {
  viewer: Viewer,
}

const UserDropdown = ({ viewer: { name, avatarUrl } }: Props) => {
  const signOut = useCallback(() => {
    window.location.href = '/logout'
  })

  return (
    <div className={styles.dropdown}>
      <details className="dropdown details-reset details-overlay d-inline-block">
        <summary className="btn" aria-haspopup="true">
          <div className={styles.summary}>
            <Avatar
              alt={name}
              size={20}
              src={`${avatarUrl}&s=40`}
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
