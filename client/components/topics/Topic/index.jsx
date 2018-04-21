// @flow
import React from 'react'

type Props = {
  topic: {
    name: string,
    resourcePath: string,
  },
}

export default ({ topic: { name, resourcePath } }: Props) => (
  <li>
    <a href={resourcePath}>{name}</a>
    <div>{resourcePath}</div>
  </li>
)
