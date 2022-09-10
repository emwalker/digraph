import React from 'react'
import { Link } from 'found'

import UserDropdown from './UserDropdown'

type Props = {
  viewer: any,
}

export default function UserNav(props: Props) {
  return (
    <>
      <Link className="Link--primary px-2" to="/review">
        Review
      </Link>
      <UserDropdown {...props} />
    </>
  )
}
