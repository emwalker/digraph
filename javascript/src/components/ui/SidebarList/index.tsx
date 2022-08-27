import React from 'react'
import { isEmpty } from 'ramda'
import { Link } from 'found'

import { topicPath } from 'components/helpers'
import BlankslateUI from '../Blankslate'

const Blankslate = ({ placeholder }: { placeholder: string }) => (
  <BlankslateUI>
    <p>{ placeholder }</p>
  </BlankslateUI>
)

type ItemType = {
  display: string,
  id: string,
} | null

type ItemListProps = {
  items: ItemType[],
}

const renderItem = () => (item: ItemType) => {
  if (!item) return null
  const { id, display } = item

  const to = {
    pathname: topicPath(id),
    state: {
      itemTitle: display,
    },
  }

  return (
    <li
      className="Box-row"
      key={id}
    >
      <Link to={to} className="Box-row-link">
        { display }
      </Link>
    </li>
  )
}

const ItemList = ({ items }: ItemListProps) => {
  const render = renderItem()
  return (
    <ul>
      { items.map(render) }
    </ul>
  )
}

type Props = {
  items: ItemType[],
  placeholder: string,
  title: string,
}

export default ({ items, placeholder, title }: Props) => (
  <div className="Box Box--condensed mb-3">
    <div className="Box-header">
      <span className="Box-title">{title}</span>
    </div>
    { isEmpty(items)
      ? <Blankslate placeholder={placeholder} />
      : <ItemList items={items} />}
  </div>
)
