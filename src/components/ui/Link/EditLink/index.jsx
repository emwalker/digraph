// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { LinkType, Relay } from 'components/types'
import EditLinkForm from './EditLinkForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    orgLogin: string,
    relay: Relay,
    view: {
      link: LinkType,
    },
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
      relay={props.relay}
      toggleForm={toggleForm}
    />
  )
}

type Props = {
  isOpen: boolean,
  link: LinkType,
  orgLogin: string,
  relay: Relay,
  toggleForm: Function,
}

const EditLink = ({ isOpen, link, orgLogin, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditLinkQuery(
        $orgLogin: String!,
        $repoName: String,
        $repoIds: [ID!],
        $linkId: ID!,
      ) {
        view(
          currentOrganizationLogin: $orgLogin,
          currentRepositoryName: $repoName,
          repositoryIds: $repoIds,
        ) {
          link(id: $linkId) {
            ...EditLinkForm_link
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      linkId: link.id,
      repoIds: [],
    }}
    render={renderer({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditLink
