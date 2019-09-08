// @flow
import React, { type Node } from 'react'

type Props = {
  children: Node,
}

const Content = ({ children }: Props) => (
  <div className="col-9 float-left pl-4">
    { children }
  </div>
)

export default Content
