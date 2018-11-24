// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { LinkType, OrganizationType } from 'components/types'
import EditLinkForm from './EditLinkForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    organization: {
      link: LinkType,
    },
  },
}

/* eslint react/prop-types: 0 */

const renderer = ({ isOpen, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.organization)
    return null

  return (
    <EditLinkForm
      isOpen={isOpen}
      link={props.organization.link}
      organization={props.organization}
      toggleForm={toggleForm}
    />
  )
}

type Props = {
  isOpen: boolean,
  link: LinkType,
  organization: OrganizationType,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
}

const EditLink = ({ isOpen, link, organization, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditLinkQuery($organizationId: ID!, $linkId: ID!) {
        organization(id: $organizationId) {
          ...EditLinkForm_organization

          link(id: $linkId) {
            ...EditLinkForm_link
          }
        }
      }
    `}
    variables={{
      linkId: link.id,
      organizationId: organization.id,
    }}
    render={renderer({ isOpen, toggleForm })}
  />
)

export default EditLink
