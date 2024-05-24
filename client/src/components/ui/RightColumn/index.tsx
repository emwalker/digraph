import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

export default function RightColumn({ children }: Props) {
  return (
    <div className="col-lg-4 col-md-6 col-12 float-right pb-3">
      { children }
    </div>
  )
}
