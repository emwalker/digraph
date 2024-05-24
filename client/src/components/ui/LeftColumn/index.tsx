import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
}

export default function LeftColumn({ children }: Props) {
  return (
    <div className="col-lg-8 col-md-6 col-12 float-left">
      { children }
    </div>
  )
} 
