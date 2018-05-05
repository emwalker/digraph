// @flow
import React from 'react'
import type { Node } from 'react'

import ItemList from '../ItemList'

type Props = {
  children: Node,
  items: Object[],
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
      <ItemList
        items={items}
        placeholder="There are no items in this list."
      />
    </div>
  </div>
)
