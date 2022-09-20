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

const query = graphql`
  query EditLinkContainerQuery(
    $linkId: String!,
    $repoIds: [ID!],
    $viewerId: ID!,
  ) {
    view(
      repoIds: $repoIds,
      viewerId: $viewerId,
    ) {
      link(id: $linkId) {
        repoLinks {
          ...EditLinkForm_repoLink
        }
      }
    }
  }
`

export default function EditLinkContainer({ isOpen, link, toggleForm }: Props) {
  const environment = useRelayEnvironment()

  return (
    <QueryRenderer
      environment={environment}
      query={query}
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
