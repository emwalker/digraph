// @flow
import React from 'react'
import type { Node } from 'react'
import { ListGroup, ListGroupItem } from 'reactstrap'

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
    <h1>{title}</h1>
    <div className="row">
      <div className="col">
        <ListGroup>
          {items.map(({ id, display, resourcePath }) => (
            <ListGroupItem
              key={id}
              tag="a"
              href={resourcePath}
            >
              { display }
            </ListGroupItem>
          ))}
        </ListGroup>
      </div>
      <div className="col-5">
        { children }
      </div>
    </div>
  </div>
)
