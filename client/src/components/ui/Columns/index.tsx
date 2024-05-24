import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

export default function Columns({ children }: Props) {
  return (
    <div className="gutter">
      { children }
    </div>
  )
}
