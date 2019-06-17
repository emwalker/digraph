// @flow
import React, { Fragment } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'

import type { UserType } from 'components/types'
import UserDropdown from './UserDropdown'

type Props = {
  viewer: UserType,
}

const UserNav = ({ viewer }: Props) => (
  <Fragment>
    <Link className="text-gray-dark px-2" to="/review">
      Review
    </Link>
    <UserDropdown viewer={viewer} />
  </Fragment>
)

export const UnwrappedUserNav = UserNav

export default createFragmentContainer(UserNav, {
  viewer: graphql`
    fragment UserNav_viewer on User {
      ...UserDropdown_viewer
    }
  `,
})
