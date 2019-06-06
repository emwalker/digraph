// @flow
import React from 'react'
import { Link } from 'found'
import isExternal from 'is-url-external'

import type { LocationDescriptor } from 'components/types'

type Props = {
  children: Iterable<React$Node> | string,
  className: string,
  to: LocationDescriptor,
}

const LinkOrA = ({ children, className, to }: Props) => {
  if (isExternal(to.pathname)) return <a href={to.pathname} className={className}>{ children }</a>
  return <Link to={to} className={className}>{ children }</Link>
}

export default LinkOrA
