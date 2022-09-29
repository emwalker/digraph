/* eslint-disable */
// @ts-nocheck

import React from 'react'
import { useQueryLoader } from 'react-relay'

import RefetchQuery from '../EditRepoLinkParentTopicsRefetchQuery'

const repoLinkFragment = graphql`
  fragment EditRepoLinkParentTopics_repoLink on RepoLink {
    link {
      id
    }

    selectedTopics: parentTopics {
      edges {
        node {
          value: id
          label: displayName
        }
      }
    }
  }
`

type Props = {
  linkId: string,
  viewerId: string,
  selectedRepoId: string,
}

export default function ParentTopics({ selectedRepoId, viewerId, ...rest }: Props) {
  return (
    <div>Parent topic list</div>
  )
}
