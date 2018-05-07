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
  id: string,
  resourcePath: string,
}

const Blankslate = ({ message }: BlankslateProps) => (
  <BlankslateUI>
    <p>{message}</p>
  </BlankslateUI>
)

const renderItem = ({
  __typename, id, resourcePath, ...props
}: ItemType) => {
  const Form = __typename === 'Topic'
    ? () => <div>Edit topic</div>
    : EditLink

  return (
    <Item
      id={id}
      key={resourcePath}
      resourcePath={resourcePath}
      FormComponent={Form}
      {...props}
    />
  )
}

type Props = {
  items: Array<ItemType>,
  // eslint-disable-next-line react/no-unused-prop-types
  placeholder: string,
}

const List = ({ items }: Props) => (
  <div className="Box">
    <ul>
      { items.map(renderItem) }
    </ul>
  </div>
)

export default ({ items, placeholder }: Props) => (isEmpty(items)
  ? <Blankslate message={placeholder} />
  : <List items={items} />)
