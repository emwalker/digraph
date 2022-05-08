import React from 'react'
import { QueryRenderer, graphql, RelayProp } from 'react-relay'

import makeEditLink from './EditLink'

type Link = {
  id: string,
}

type Props = {
  isOpen: boolean,
  link: Link,
  orgLogin: string,
  relay: RelayProp,
  toggleForm: () => void,
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
            ...EditLinkForm_link
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
