// @flow
import React from 'react'
import { Link } from 'found'
import isExternal from 'is-url-external'

type Props = {
  children: Node,
  className: string,
  to: string,
}

const LinkOrA = ({ children, className, to }: Props) => {
  if (isExternal(to))
    return <a href={to} className={className}>{ children }</a>
  return <Link to={to} className={className}>{ children }</Link>
}

export default LinkOrA
