import React from 'react'

import {
  EditTopicContainerQueryResponse as Response,
} from '__generated__/EditTopicContainerQuery.graphql'
import EditTopicForm from './EditTopicForm'

type ContainerViewType = Response['view']

type CallerProps = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: () => void,
}

type RenderArgs = {
  error: Error | null,
  props?: unknown
}

type RenderProps = {
  view: ContainerViewType
}

export default ({ isOpen, orgLogin, toggleForm }: CallerProps) => (
  { error, props: renderProps }: RenderArgs,
) => {
  if (error) return <div>{error.message}</div>
  if (!renderProps) return null
  const { view } = renderProps as RenderProps
  return (
    view?.topic && (
      <EditTopicForm
        isOpen={isOpen}
        toggleForm={toggleForm}
        topic={view.topic}
        orgLogin={orgLogin}
      />
    )
  )
}
