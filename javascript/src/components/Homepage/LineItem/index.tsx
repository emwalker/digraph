import React from 'react'
import { graphql, useFragment } from 'react-relay'
import Markdown from 'react-markdown'
import TimeAgo from 'react-timeago'

import { LineItem_item$key } from '__generated__/LineItem_item.graphql'

type Props = {
  item: LineItem_item$key,
}

export default function LineItem(props: Props) {
  const item = useFragment(
    graphql`
      fragment LineItem_item on ActivityLineItem {
        description
        createdAt
      }
    `,
    props.item,
  )
  const { createdAt, description } = item

  return (
    <div className="lineItem my-3">
      <Markdown className="lineItemDescription">{description}</Markdown>
      <TimeAgo className="lineItemTimeAgo" date={createdAt as string} />
    </div>
  )
}
