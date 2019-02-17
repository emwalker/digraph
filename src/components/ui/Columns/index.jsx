// @flow
import React from 'react'

type Props = {
  children: React$Node | Iterable<React$Node>,
}

const Columns = ({ children }: Props) => (
  <div className="gutter px-md-0">
    { children }
  </div>
)

export default Columns
