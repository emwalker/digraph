import React, { ReactNode } from 'react'
import { Link } from 'found'

import isExternal from 'utils/isExternal'
import { LocationType } from 'components/types'

type Props = {
  children: ReactNode,
  className: string,
  to: LocationType,
}

const LinkOrA = ({ children, className, to }: Props) => {
  if (isExternal(to.pathname)) return <a href={to.pathname} className={className}>{ children }</a>
  return <Link to={to} className={className}>{ children }</Link>
}

export default LinkOrA
