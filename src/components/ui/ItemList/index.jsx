// @flow
import React from 'react'
import { isEmpty } from 'ramda'

import Item, { EditLink } from './Item'
import BlankslateUI from '../Blankslate'

type BlankslateProps = {
  message: string,
}

type ItemType = {
  __typename: string,
  display: string,
  resourceId: string,
  resourcePath: string,
}

const Blankslate = ({ message }: BlankslateProps) => (
  <BlankslateUI>
    <p>{message}</p>
  </BlankslateUI>
)

const renderItem = (
  {
    __typename, resourceId, ...itemProps
  }: ItemType,
  props: Object,
) => {
  const Form = __typename === 'Topic'
    ? () => <div>Edit topic</div>
    : EditLink

  return (
    <Item
      id={resourceId}
      key={resourceId}
      FormComponent={Form}
      resourceId={resourceId}
      {...props}
      {...itemProps}
    />
  )
}

type Props = {
  items: Array<ItemType>,
  // eslint-disable-next-line react/no-unused-prop-types
  placeholder: string,
}

const List = ({ items, ...props }: Props) => (
  <div className="Box">
    <ul>
      { items.map(item => renderItem(item, props)) }
    </ul>
  </div>
)

export default ({ items, placeholder, ...props }: Props) => (isEmpty(items)
  ? <Blankslate message={placeholder} />
  : <List items={items} {...props} />)
