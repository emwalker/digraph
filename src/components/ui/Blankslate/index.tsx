import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode,
  title?: string | undefined,
}

const Blankslate = ({ children, title }: Props) => (
  <div className="blankslate">
    {title && <h3>{title}</h3>}
    { children }
  </div>
)

Blankslate.defaultProps = {
  title: null,
}

export default Blankslate
