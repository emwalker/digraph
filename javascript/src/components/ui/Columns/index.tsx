import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

const Columns = ({ children }: Props) => (
  <div className="gutter">
    { children }
  </div>
)

export default Columns
