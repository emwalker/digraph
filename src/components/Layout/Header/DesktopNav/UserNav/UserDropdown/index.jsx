// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { UserType } from 'components/types'

type Props = {
  viewer: UserType,
}

const UserDropdown = ({ viewer: { name, avatarUrl } }: Props) => (
  <summary className="d-inline">
    <a className="text-gray-dark px-2" href="/logout">Sign out</a>
    <img
      alt={name}
      className="avatar"
      height="20"
      src={`${avatarUrl}&s=40`}
      width="20"
    />
  </summary>
)

export const UnwrappedUserDropdown = UserDropdown

export default createFragmentContainer(UserDropdown, {
  viewer: graphql`
    fragment UserDropdown_viewer on User {
      name
      avatarUrl
    }
  `,
})
