import React from 'react'
import Markdown from 'react-markdown'
import TimeAgo from 'react-timeago'

type Props = {
  item: {
    createdAt: unknown,
    description: string,
  },
}

export default ({ item }: Props) => (
  <div className="Box-row container-lg clearfix activity-line-item">
    <div className="float-left col-lg-2 col-4 pr-2">
      <TimeAgo date={item.createdAt as string} />
    </div>
    <div className="float-left col-lg-10 col-8">
      <Markdown>{item.description}</Markdown>
    </div>
  </div>
)
