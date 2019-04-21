// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children: Node,
}

const Columns = ({ children }: Props) => (
  <div className="gutter px-md-0">
    { children }
  </div>
)

export default Columns
