import React from 'react'
import { Link } from 'found'

type Props = {
  displayName: string,
  to: {
    pathname: string,
  },
}

export default ({ displayName, to }: Props) => (
  <Link to={to} className="Box-row-link" key={to.pathname}>
    <span
      className="Label Label--outline Label--outline-gray"
    >
      {displayName}
    </span>
  </Link>
)
