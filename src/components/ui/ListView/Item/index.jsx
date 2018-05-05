// @flow
import React from 'react'
import { pathOr } from 'ramda'

const edges = pathOr([], ['edges'])

type BadgeProps = {
  node: {
    name: string,
    resourcePath: string,
  }
}

const renderTopic = ({ node: { name, resourcePath } }: BadgeProps) => (
  <a className="Box-row-link" href={resourcePath} key={resourcePath}>
    <span
      className="Label swatch-green"
    >
      {name}
    </span>
  </a>
)

type Props = {
  display: string,
  resourcePath: string,
  topics: Array[],
}

export default ({ display, resourcePath, topics }: Props) => (
  <li
    className="Box-row"
    key={resourcePath}
  >
    <a className="Box-row-link" href={resourcePath}>
      { display }
    </a>
    { edges(topics).map(renderTopic) }
  </li>
)
