import React from 'react'

import {
  EditLinkContainerQueryResponse as Response,
} from '__generated__/EditLinkContainerQuery.graphql'
import EditLinkForm from './EditLinkForm'

type ContainerViewType = Response['view']

type CallerProps = {
  isOpen: boolean,
  toggleForm: () => void,
}

type RenderArgs = {
  error: Error | null,
  props?: unknown
}

type RenderProps = {
  view: ContainerViewType
}

export default ({ isOpen, toggleForm }: CallerProps) => (
  { error, props: renderProps }: RenderArgs,
) => {
  if (error) return <div>{error.message}</div>
  if (!renderProps) return null
  const { view } = renderProps as RenderProps
  return (
    view?.link && (
      <EditLinkForm
        isOpen={isOpen}
        toggleForm={toggleForm}
        link={view.link}
      />
    )
  )
}
