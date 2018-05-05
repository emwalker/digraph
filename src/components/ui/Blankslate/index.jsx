// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children: Node,
  title: string,
}

export default ({ children, title }: Props) => (
  <div className="blankslate">
    <h3>{title}</h3>
    { children }
  </div>
)
