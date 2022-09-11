import React from 'react'
import { PreloadedQuery, usePreloadedQuery } from 'react-relay'

import EditTopicQuery from '../EditTopicQuery'
import EditTopic from '../EditTopicContainer/EditTopic'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'

type Props = {
  queryRef: PreloadedQuery<EditTopicQueryType>,
  toggleForm: () => void,
}

export default function EditTopicOuter(props: Props) {
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
