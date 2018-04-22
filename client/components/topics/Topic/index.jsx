// @flow
import React from 'react'

type Props = {
  topic: {
    name: string,
    resourceIdentifier: string,
  },
}

export default ({ topic: { name, resourceIdentifier } }: Props) => (
  <li>
    <a href={resourceIdentifier}>{name}</a>
    <div>{resourceIdentifier}</div>
  </li>
)
