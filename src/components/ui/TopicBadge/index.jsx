// @flow
import React from 'react'

type Props = {
  name: string,
  resourcePath: string,
}

export default ({ name, resourcePath }: Props) => (
  <a className="Box-row-link" href={resourcePath} key={resourcePath}>
    <span
      className="Label Label--outline Label--outline-gray"
    >
      {name}
    </span>
  </a>
)
