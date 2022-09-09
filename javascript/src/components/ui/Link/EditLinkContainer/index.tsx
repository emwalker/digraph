import React from 'react'
import { QueryRenderer, graphql, RelayProp } from 'react-relay'

import makeEditLink from './EditLink'

type Link = {
  id: string,
}

type Props = {
  isOpen: boolean,
  link: Link,
  relay: RelayProp,
  toggleForm: () => void,
}

const EditLinkContainer = ({ isOpen, link, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
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

export default EditLinkContainer
