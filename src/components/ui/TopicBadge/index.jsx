// @flow
import React from 'react'
import { Link } from 'found'

type Props = {
  name: string,
  to: {
    pathname: string,
  },
}

export default ({ name, to }: Props) => (
  <Link to={to} className="Box-row-link" key={to.pathname}>
    <span
      className="Label Label--outline Label--outline-gray"
    >
      {name}
    </span>
  </Link>
)
