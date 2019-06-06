// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LinkType, Relay, UserType, ViewType } from 'components/types'
import { liftNodes } from 'utils'
import EditLinkForm from './EditLinkForm'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: Function,
}

type PropsType = {
  error: ?Object,
  orgLogin: string,
  relay: Relay,
  link: LinkType,
  view: ViewType,
  viewer: UserType,
} & Props

type RenderProps = {
  error: ?Object,
  props: ?PropsType,
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const EditLink = (props: PropsType) => {
  const { error, isOpen, orgLogin, relay, toggleForm, link, viewer } = props

  if (error) return <div>{error.message}</div>

  if (!link) return null

  return (
    <EditLinkForm
      availableTopics={liftNodes(link.availableTopics)}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={relay}
      selectedTopics={liftNodes(link.selectedTopics)}
      toggleForm={toggleForm}
      link={link}
      viewer={viewer}
    />
  )
}

const Wrapped = createFragmentContainer(EditLink, {
  link: graphql`
    fragment EditLink_link on Link {
      ...EditLinkForm_link
    }
  `,
})

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => {
  if (!props) return null

  const { view, viewer } = props

  return (
    <Wrapped
      error={error}
      isOpen={isOpen}
      orgLogin={orgLogin}
      toggleForm={toggleForm}
      link={view.link}
      relay={props.relay}
      view={view}
      viewer={viewer}
    />
  )
}
