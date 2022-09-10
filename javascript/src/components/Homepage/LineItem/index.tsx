import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import Markdown from 'react-markdown'
import TimeAgo from 'react-timeago'
import classNames from 'classnames'

import { LineItem_item$data as ItemType } from '__generated__/LineItem_item.graphql'

type Props = {
  item: ItemType,
}

const LineItem = ({ item }: Props) => {
  const { createdAt, description } = item

  return (
    <div className={classNames('lineItem', 'my-3')}>
      <Markdown className="lineItemDescription">{description}</Markdown>
      <TimeAgo className="lineItemTimeAgo" date={createdAt as string} />
    </div>
  )
}

export default createFragmentContainer(LineItem, {
  item: graphql`
    fragment LineItem_item on ActivityLineItem {
      description
      createdAt
    }
  `,
})
