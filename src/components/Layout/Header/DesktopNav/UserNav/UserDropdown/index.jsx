// @flow
import React, { useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Avatar, Dropdown } from '@primer/components'
import { Link } from 'found'

import type { UserType } from 'components/types'
import styles from './styles.module.css'

type Props = {
  viewer: UserType,
}

const UserDropdown = ({ viewer: { name, avatarUrl } }: Props) => {
  const avatar = (
    <Avatar
      alt={name}
      size={20}
      src={`${avatarUrl}&s=40`}
    />
  )

  const signOut = useCallback(() => {
    window.location.href = '/logout'
  })

  return (
    <span className={styles.dropdown}>
      <Dropdown
        backgroundColor="transparent"
        title={avatar}
      >
        <Dropdown.Menu direction="sw">
          <Dropdown.Item>
            <Link className={styles.itemLink} to="/settings/account">Account</Link>
          </Dropdown.Item>
          <Dropdown.Item>
            <a className={styles.itemLink} onClick={signOut} href="/logout">Sign out</a>
          </Dropdown.Item>
        </Dropdown.Menu>
      </Dropdown>
    </span>
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
