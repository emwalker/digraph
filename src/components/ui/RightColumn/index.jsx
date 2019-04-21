// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children: Node,
}

const RightColumn = ({ children }: Props) => (
  <div className="col-lg-4 col-md-6 col-12 float-right pb-3">
    { children }
  </div>
)

export default RightColumn
