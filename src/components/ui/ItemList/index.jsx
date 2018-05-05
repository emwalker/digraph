// @flow
import React from 'react'
import { isEmpty } from 'ramda'

import Item from './Item'
import BlankslateUI from '../Blankslate'

type BlankslateProps = {
  message: string,
}

const Blankslate = ({ message }: BlankslateProps) => (
  <BlankslateUI>
    <p>{message}</p>
  </BlankslateUI>
)

type ItemType = {
  id: string,
  display: string,
  resourcePath: string,
}

type Props = {
  items: Array<ItemType>,
  // eslint-disable-next-line react/no-unused-prop-types
  placeholder: string,
}

const List = ({ items }: Props) => (
  <div className="Box">
    <ul>
      { items.map(({ resourcePath, ...props }) => (
        <Item
          key={resourcePath}
          resourcePath={resourcePath}
          {...props}
        />
      ))
      }
    </ul>
  </div>
)

export default ({ items, placeholder }: Props) => (isEmpty(items)
  ? <Blankslate message={placeholder} />
  : <List items={items} />)
