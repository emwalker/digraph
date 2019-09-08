// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children: Node,
}

const Columns = ({ children }: Props) => (
  <div className="gutter">
    { children }
  </div>
)

export default Columns
