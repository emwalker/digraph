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

  // FIXME
  const repoLinks = view?.link?.repoLinks || []
  if (repoLinks.length < 1) return null

  return (
    <EditLinkForm
      isOpen={isOpen}
      toggleForm={toggleForm}
      repoLink={repoLinks[0]}
    />
  )
}
