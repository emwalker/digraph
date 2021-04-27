import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

const LeftColumn = ({ children }: Props) => (
  <div className="col-lg-8 col-md-6 col-12 float-left">
    { children }
  </div>
)

export default LeftColumn
