import React from 'react'
import { useQueryLoader, PreloadedQuery, usePreloadedQuery } from 'react-relay'

import EditTopicQuery from '../EditTopicQuery'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'
import EditTopic from './EditTopic'

type Props = {
  queryRef: PreloadedQuery<EditTopicQueryType>,
}

function UnwrapData(props: Props) {
  const data = usePreloadedQuery<EditTopicQueryType>(EditTopicQuery, props.queryRef)

  if (!data.view || !data.view.topic) return null

  return (
    <EditTopic
      topic={data.view.topic}
      viewer={data.view.viewer}
    />
  )
}

export default function EditTopicContainer(props: Props) {
  const queryRef =
    useQueryLoader<EditTopicQueryType>(EditTopicQuery, props.queryRef)[0]

  if (!queryRef) return null

  return <UnwrapData queryRef={queryRef} />
}
