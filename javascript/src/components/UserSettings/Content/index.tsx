import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

const Content = ({ children }: Props) => (
  <div className="col-9 float-left pl-4">
    { children }
  </div>
)

export default Content
