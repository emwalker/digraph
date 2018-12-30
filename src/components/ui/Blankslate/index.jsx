// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children: Node,
  title?: ?string,
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
