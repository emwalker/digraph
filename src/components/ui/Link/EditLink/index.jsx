// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { LinkType } from 'components/types'
import EditLinkForm from './EditLinkForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    view: {
      link: LinkType,
    },
    orgLogin: string,
  },
}

/* eslint react/prop-types: 0 */

const renderer = ({ isOpen, orgLogin, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.view)
    return null

  return (
    <EditLinkForm
      isOpen={isOpen}
      link={props.view.link}
      orgLogin={orgLogin}
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

const EditLink = ({ isOpen, link, orgLogin, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditLinkQuery($repoIds: [ID!], $linkId: ID!) {
        viewer {
          ...EditLinkForm_viewer
        }

        view(repositoryIds: $repoIds) {
          link(id: $linkId) {
            ...EditLinkForm_link
          }
        }
      }
    `}
    variables={{
      linkId: link.id,
      repoIds: [],
    }}
    render={renderer({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditLink
