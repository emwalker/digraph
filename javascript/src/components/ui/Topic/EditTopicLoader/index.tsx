import React, { Suspense, useEffect, useState } from 'react'
import { loadQuery, PreloadedQuery, useRelayEnvironment } from 'react-relay'

import EditTopicContainer from '../EditTopicContainer'
import EditTopicQuery from '../EditTopicQuery'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'

type Props = {
  toggleForm: () => void,
  topicId: string,
  viewerId: string,
}

export default function EditTopicLoader(props: Props) {
  const environment = useRelayEnvironment()
  const sentinel = {} as PreloadedQuery<EditTopicQueryType>
  const [queryRef, setQueryRef] = useState(sentinel)

  useEffect(() => {
    const newQueryRef = loadQuery<EditTopicQueryType>(environment, EditTopicQuery, {
      topicId: props.topicId,
      viewerId: props.viewerId,
    }, { fetchPolicy: 'network-only' })
    setQueryRef(newQueryRef)
  }, [setQueryRef])

  return (
    <Suspense fallback={<div>Loading form ...</div>}>
      {queryRef !== sentinel && (
        <EditTopicContainer
          toggleForm={props.toggleForm}
          queryRef={queryRef}
        />
      )}
    </Suspense>
  )
}
