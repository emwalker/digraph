import React from 'react'
import { useQueryLoader, PreloadedQuery, usePreloadedQuery } from 'react-relay'

import EditTopicQuery from '../EditTopicQuery'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'
import EditTopic from './EditTopic'

type Props = {
  queryRef: PreloadedQuery<EditTopicQueryType>,
  toggleForm: () => void,
}

function Outer(props: Props) {
  const data = usePreloadedQuery<EditTopicQueryType>(EditTopicQuery, props.queryRef)

  if (!data.view || !data.view.topic) return null
  const topic = data.view.topic

  return (
    <EditTopic
      toggleForm={props.toggleForm}
      topic={topic}
      viewer={data.view.viewer}
    />
  )
}

export default function EditTopicContainer(props: Props) {
  const queryRef =
    useQueryLoader<EditTopicQueryType>(EditTopicQuery, props.queryRef)[0]

  if (!queryRef) return null

  return (
    <Outer
      queryRef={queryRef}
      toggleForm={props.toggleForm}
    />
  )
}
