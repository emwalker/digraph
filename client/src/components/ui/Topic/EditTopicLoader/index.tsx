import React, { Suspense, useEffect, useState } from 'react'
import { loadQuery, PreloadedQuery, useRelayEnvironment } from 'react-relay'

import EditTopicContainer from '../EditTopicContainer'
import EditTopicQuery from '../EditTopicQuery'
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql'

type Props = {
  topicId: string,
  viewerId: string,
}

export default function EditTopicLoader({ topicId, viewerId }: Props) {
  const environment = useRelayEnvironment()
  const emptyQueryRef = {} as PreloadedQuery<EditTopicQueryType>
  const [queryRef, setQueryRef] = useState(emptyQueryRef)

  useEffect(() => {
    const newQueryRef = loadQuery<EditTopicQueryType>(
      environment,
      EditTopicQuery,
      { topicId, viewerId },
    )
    setQueryRef(newQueryRef)
  }, [setQueryRef])

  return (
    <Suspense fallback={<div>Loading form ...</div>}>
      {queryRef !== emptyQueryRef && <EditTopicContainer queryRef={queryRef} />}
    </Suspense>
  )
}
