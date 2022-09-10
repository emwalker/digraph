import React from 'react'
import { QueryRenderer, graphql, useRelayEnvironment } from 'react-relay'

import makeEditLink from './EditLink'

type Link = {
  id: string,
}

type Props = {
  isOpen: boolean,
  link: Link,
  toggleForm: () => void,
}

export default function EditLinkContainer({ isOpen, link, toggleForm }: Props) {
  const environment = useRelayEnvironment()

  return (
    <QueryRenderer
      environment={environment}
      query={graphql`
        query EditLinkContainerQuery(
          $viewerId: ID!,
          $repoIds: [ID!],
          $linkId: String!,
        ) {
          view(
            viewerId: $viewerId,
            repositoryIds: $repoIds,
          ) {
            link(id: $linkId) {
              repoLinks {
                ...EditLinkForm_repoLink
              }
            }
          }
        }
      `}
      variables={{
        repoName: null,
        linkId: link.id,
        viewerId: '',
        repoIds: [],
      }}
      render={makeEditLink({ isOpen, toggleForm })}
    />
  )
}
