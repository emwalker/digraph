// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import makeEditLink from './EditLink'

type Link = {
  +id: string,
}

type Props = {
  isOpen: boolean,
  link: Link,
  orgLogin: string,
  relay: Relay,
  toggleForm: Function,
}

const EditLinkContainer = ({ isOpen, link, orgLogin, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditLinkContainerQuery(
        $viewerId: ID!,
        $orgLogin: String!,
        $repoName: String,
        $repoIds: [ID!],
        $linkId: ID!,
      ) {
        view(
          viewerId: $viewerId,
          currentOrganizationLogin: $orgLogin,
          currentRepositoryName: $repoName,
          repositoryIds: $repoIds,
        ) {
          link(id: $linkId) {
            ...EditLink_link
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      linkId: link.id,
      viewerId: '',
      repoIds: [],
    }}
    render={makeEditLink({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditLinkContainer
