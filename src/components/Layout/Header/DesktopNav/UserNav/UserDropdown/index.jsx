// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Avatar, Dropdown } from '@primer/components'

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

  return (
    <span className={styles.dropdown}>
      <Dropdown
        backgroundColor="transparent"
        title={avatar}
      >
        <Dropdown.Menu direction="sw">
          <Dropdown.Item>Settings</Dropdown.Item>
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
