// @flow
import React from 'react'
import type { Node } from 'react'
import { isEmpty } from 'ramda'

import Item from './Item'
import BlankslateUI from '../Blankslate'

const Blankslate = () => (
  <BlankslateUI>
    <p>There are no items in this list.</p>
  </BlankslateUI>
)

type ItemType = {
  id: string,
  display: string,
  resourcePath: string,
}

type ItemListProps = {
  items: Array<ItemType>,
}

const ItemList = ({ items }: ItemListProps) => (
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

type Props = {
  children: Node,
  items: Array<ItemType>,
  title: string,
}

export default ({ children, items, title }: Props) => (
  <div>
    <div className="Subhead">
      <div className="Subhead-heading">{title}</div>
    </div>
    <div className="one-third column pl-0">
      { children }
    </div>
    <div className="two-thirds column pr-0">
      { isEmpty(items)
        ? <Blankslate />
        : <ItemList items={items} />
      }
    </div>
  </div>
)
