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
    __typename, resourceId, ...item
  }: ItemType,
  props: Object,
) => {
  const isTopic = __typename === 'Topic'
  const Form = isTopic ? () => null : EditLink
  const className = isTopic ? 'Box-row-topic' : 'Box-row-link'

  return (
    <Item
      __typename={__typename}
      className={className}
      id={resourceId}
      key={resourceId}
      FormComponent={Form}
      resourceId={resourceId}
      item={{ resourceId, ...item }}
      {...props}
      {...item}
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
