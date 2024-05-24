import React from 'react'

import UserDropdown from './UserDropdown'

type Props = {
  viewer: any,
}

export default function UserNav(props: Props) {
  return (
    <UserDropdown {...props} />
  )
}
