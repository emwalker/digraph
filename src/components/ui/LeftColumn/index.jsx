// @flow
import React from 'react'

type Props = {
  children: React$Node | Iterable<React$Node>,
}

const LeftColumn = ({ children }: Props) => (
  <div className="col-lg-8 col-md-6 col-12 float-left digraph-anim-fade-in">
    { children }
  </div>
)

export default LeftColumn
