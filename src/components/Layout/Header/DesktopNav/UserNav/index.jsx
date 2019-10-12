// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'

import UserDropdown from './UserDropdown'
import type { UserNav_viewer as Viewer } from './__generated__/UserNav_viewer.graphql'

type Props = {
  viewer: Viewer,
}

const UserNav = ({ viewer }: Props) => (
  <>
    <Link className="text-gray-dark px-2" to="/review">
      Review
    </Link>
    <UserDropdown viewer={viewer} />
  </>
)

export const UnwrappedUserNav = UserNav

export default createFragmentContainer(UserNav, {
  viewer: graphql`
    fragment UserNav_viewer on User {
      ...UserDropdown_viewer
    }
  `,
})
