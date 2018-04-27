// @flow
import React from 'react'

type Props = {
  topic: {
    name: string,
    resourceId: string,
  },
}

export default ({ topic: { name, resourceId } }: Props) => (
  <li>
    <div>{name}</div>
    <div>{resourceId}</div>
  </li>
)
