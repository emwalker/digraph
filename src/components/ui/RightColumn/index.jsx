// @flow
import React from 'react'

type Props = {
  children: React$Node | Iterable<React$Node>,
}

const RightColumn = ({ children }: Props) => (
  <div className="col-lg-4 col-md-6 col-12 float-right pb-3">
    { children }
  </div>
)

export default RightColumn
