import React, { Suspense, useEffect, useState } from 'react'
import { loadQuery, PreloadedQuery, useRelayEnvironment } from 'react-relay'

import EditLinkQuery from '../EditLinkQuery'
import EditLinkContainer from '../EditLinkContainer'
import { EditLinkQuery as EditLinkQueryType } from '__generated__/EditLinkQuery.graphql'

type Props = {
  linkId: string,
  viewerId: string,
}

export default function EditLinkLoader({ linkId, viewerId }: Props) {
  const environment = useRelayEnvironment()
  const emptyQueryRef = {} as PreloadedQuery<EditLinkQueryType>
  const [queryRef, setQueryRef] = useState(emptyQueryRef)

  useEffect(() => {
    const newQueryRef = loadQuery<EditLinkQueryType>(
      environment,
      EditLinkQuery,
      { linkId, viewerId },
    )
    setQueryRef(newQueryRef)
  }, [setQueryRef, viewerId])

  return (
    <Suspense fallback={<div>Loading ...</div>}>
      {queryRef !== emptyQueryRef && <EditLinkContainer queryRef={queryRef} />}
    </Suspense>
  )
}
