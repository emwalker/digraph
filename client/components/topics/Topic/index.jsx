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
    <div><a href={resourceId}>{name}</a></div>
    <div>{resourceId}</div>
  </li>
)
