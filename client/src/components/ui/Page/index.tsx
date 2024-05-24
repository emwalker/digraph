import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

export default function Page({ children }: Props) {
  return (
    <div className="container-lg clearfix my-5 px-3 px-md-6 px-lg-3">
      { children }
    </div>
  )
}
