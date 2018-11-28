// @flow
import React from 'react'
import { isEmpty } from 'ramda'
import { Link } from 'found'

import BlankslateUI from '../Blankslate'

const Blankslate = () => (
  <BlankslateUI>
    <p>There are no parent topics for this topic.</p>
  </BlankslateUI>
)

type ItemType = {
  display: string,
  resourcePath: string,
}

type ItemListProps = {
  items: ItemType[],
}

const ItemList = ({ items }: ItemListProps) => (
  <ul>
    {items.map(({ resourcePath, display }) => (
      <li
        className="Box-row"
        key={resourcePath}
      >
        <Link to={resourcePath} className="Box-row-link">
          { display }
        </Link>
      </li>
    ))}
  </ul>
)

type Props = {
  items: ItemType[],
  title: string,
}

export default ({ items, title }: Props) => (
  <div className="Box Box--condensed">
    <div className="Box-header">
      <span className="Box-title">{title}</span>
    </div>
    { isEmpty(items)
      ? <Blankslate />
      : <ItemList items={items} />
    }
  </div>
)
