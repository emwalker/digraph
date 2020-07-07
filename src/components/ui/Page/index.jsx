// @flow
import React, { type Node } from 'react'

type Props = {
  children: Node,
}

const Page = ({ children }: Props) => (
  <div className="container-lg clearfix my-5 px-3 px-md-6 px-lg-3">
    { children }
  </div>
)

export default Page
