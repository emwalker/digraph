// @flow
import React from 'react'

import BlankslateUI from '../Blankslate'

type BlankslateProps = {
  message: string,
}

const Blankslate = ({ message }: BlankslateProps) => (
  <BlankslateUI>
    <p>{message}</p>
  </BlankslateUI>
)

type Props = {
  children: Object[],
  placeholder: string,
  hasItems: boolean,
}

export default ({ children, hasItems, placeholder }: Props) => {
  if (!hasItems)
    return <Blankslate message={placeholder} />

  return (
    <div className="Box">
      <ul>
        { children }
      </ul>
    </div>
  )
}
