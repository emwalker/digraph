// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { LinkType } from 'components/types'
import { defaultOrganizationId } from 'components/constants'
import EditLinkForm from './EditLinkForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    view: {
      link: LinkType,
    },
  },
}

/* eslint react/prop-types: 0 */

const renderer = ({ isOpen, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.view)
    return null

  return (
    <EditLinkForm
      isOpen={isOpen}
      link={props.view.link}
      toggleForm={toggleForm}
      viewer={props.viewer}
    />
  )
}

type Props = {
  isOpen: boolean,
  link: LinkType,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
}

const EditLink = ({ isOpen, link, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditLinkQuery($orgIds: [ID!], $linkId: ID!) {
        viewer {
          ...EditLinkForm_viewer
        }

        view(organizationIds: $orgIds) {
          link(id: $linkId) {
            ...EditLinkForm_link
          }
        }
      }
    `}
    variables={{
      linkId: link.id,
      orgIds: [defaultOrganizationId],
    }}
    render={renderer({ isOpen, toggleForm })}
  />
)

export default EditLink
