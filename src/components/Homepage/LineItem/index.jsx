// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import Markdown from 'react-markdown'
import TimeAgo from 'react-timeago'
import classNames from 'classnames'

import type { LineItem_item as ItemType } from './__generated__/LineItem_item.graphql'
import styles from './styles.module.css'

type Props = {
  item: ItemType,
}

const LineItem = ({ item }: Props) => {
  const { createdAt, description } = item

  return (
    <div className={classNames(styles.lineItem, 'my-3')}>
      <Markdown className={styles.description}>{description}</Markdown>
      <TimeAgo className={styles.timeAgo} date={createdAt} />
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
