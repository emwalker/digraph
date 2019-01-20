// @flow
import React from 'react'

import type { UserType } from 'components/types'

type Props = {
  viewer: UserType,
}

const ViewerDropdown = ({ viewer: { name, avatarUrl } }: Props) => (
  <summary className="d-inline">
    <a className="text-gray-dark pl-3 pr-2" href="/logout/github">Sign out</a>
    <img
      alt={name}
      className="avatar"
      height="20"
      src={`${avatarUrl}&s=40`}
      width="20"
    />
  </summary>
)

export default ViewerDropdown
