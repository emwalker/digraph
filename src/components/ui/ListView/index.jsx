// @flow
import React from 'react'
import type { Node } from 'react'

import Item from './Item'

type Props = {
  children: Node,
  items: Array<{
    id: string,
    display: string,
    resourcePath: string,
  }>,
  title: string,
}

export default ({ children, items, title }: Props) => (
  <div>
    <div className="Subhead">
      <div className="Subhead-heading">{title}</div>
    </div>
    <div className="one-third column">
      { children }
    </div>
    <div className="two-thirds column">
      <div className="Box">
        <ul>
          {items.map(({ resourcePath, ...props }) =>
            (<Item
              key={resourcePath}
              resourcePath={resourcePath}
              {...props}
            />))}
        </ul>
      </div>
    </div>
  </div>
)
