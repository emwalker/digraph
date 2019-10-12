// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import EditLinkForm from './EditLinkForm'
import type { EditLink_link as Link } from './__generated__/EditLink_link.graphql'

type View = {
  link: Link,
}

type Props = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: Function,
}

type PropsType = {
  error: ?Object,
  orgLogin: string,
  link: Link,
  view: View,
} & Props

type RenderProps = {
  error: ?Object,
  props: ?PropsType,
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const EditLink = (props: PropsType) => {
  const { error, link } = props

  if (error) return <div>{error.message}</div>

  return <EditLinkForm {...props} link={link} />
}

const Wrapped = createFragmentContainer(EditLink, {
  link: graphql`
    fragment EditLink_link on Link {
      id
      ...EditLinkForm_link
    }
  `,
})

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => (
  props && props.view && props.view.link && (
    <Wrapped
      {...props}
      error={error}
      isOpen={isOpen}
      link={props.view.link}
      orgLogin={orgLogin}
      toggleForm={toggleForm}
    />
  )
)
