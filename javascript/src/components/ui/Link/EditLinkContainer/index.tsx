import React from 'react'
import { PreloadedQuery, usePreloadedQuery, useQueryLoader } from 'react-relay'

import EditLink from './EditLink'
import EditLinkQuery from '../EditLinkQuery'
import { EditLinkQuery as EditLinkQueryType } from '__generated__/EditLinkQuery.graphql'

type Props = {
  queryRef: PreloadedQuery<EditLinkQueryType>,
  toggleForm: () => void,
}

function Outer(props: Props) {
  const data = usePreloadedQuery<EditLinkQueryType>(EditLinkQuery, props.queryRef)

  if (!data.view || !data.view.link) return null
  const link = data.view.link

  return (
    <EditLink
      toggleForm={props.toggleForm}
      link={link}
      refetch={() => {}}
      viewer={data.view.viewer}
    />
  )
}

export default function EditLinkContainer(props: Props) {
  const queryRef =
    useQueryLoader<EditLinkQueryType>(EditLinkQuery, props.queryRef)[0]

  if (!queryRef) return null

  return (
    <Outer
      queryRef={queryRef}
      toggleForm={props.toggleForm}
    />
  )
}
