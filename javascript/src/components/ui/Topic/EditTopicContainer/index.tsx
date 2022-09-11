import React from 'react'
import { useQueryLoader, PreloadedQuery } from 'react-relay'

import EditTopicOuter from '../EditTopicOuter'
import EditTopicQuery from '../EditTopicQuery'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'

type Props = {
  queryRef: PreloadedQuery<EditTopicQueryType>,
  toggleForm: () => void,
}

export default function EditTopicContainer(props: Props) {
  const queryRef =
    useQueryLoader<EditTopicQueryType>(EditTopicQuery, props.queryRef)[0]

  if (!queryRef) return null

  return (
    <EditTopicOuter
      queryRef={queryRef}
      toggleForm={props.toggleForm}
    />
  )
}
