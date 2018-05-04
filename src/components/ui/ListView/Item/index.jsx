// @flow
import React from 'react'
import { pathOr } from 'ramda'
import { Badge, ListGroupItem } from 'reactstrap'

const edges = pathOr([], ['edges'])

type BadgeProps = {
  node: {
    name: string,
    resourcePath: string,
  }
}

const renderTopic = ({ node: { name, resourcePath } }: BadgeProps) => (
  <a href={resourcePath} key={resourcePath}>
    <Badge
      color="success"
      pill
    >
      {name}
    </Badge>
  </a>
)

type Props = {
  display: string,
  resourcePath: string,
  topics: Array[],
}

export default ({ display, resourcePath, topics }: Props) => (
  <ListGroupItem
    key={resourcePath}
  >
    <a href={resourcePath}>
      { display }
    </a>
    { edges(topics).map(renderTopic) }
  </ListGroupItem>
)
