// @flow
import React from 'react'
import { Link } from 'found'

type Props = {
  name: string,
  resourcePath: string,
}

export default ({ name, resourcePath }: Props) => (
  <Link to={resourcePath} className="Box-row-link" key={resourcePath}>
    <span
      className="Label Label--outline Label--outline-gray"
    >
      {name}
    </span>
  </Link>
)
