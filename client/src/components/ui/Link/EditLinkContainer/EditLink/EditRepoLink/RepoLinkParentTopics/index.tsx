import React, { useCallback } from 'react'
import { graphql, fetchQuery, useFragment, useRelayEnvironment } from 'react-relay'

import {
  RepoLinkParentTopics_repoLink$key,
} from '__generated__/RepoLinkParentTopics_repoLink.graphql'
import RefetchQuery from '../RepoLinkParentTopicsRefetchQuery'
import {
  RepoLinkParentTopicsRefetchQuery as RefetchQueryType,
} from '__generated__/RepoLinkParentTopicsRefetchQuery.graphql'
import EditParentTopicList, { makeOptions } from 'components/ui/EditParentTopicList'
import { makeUpdateLinkParentTopicsCallback } from 'mutations/updateLinkParentTopicsMutation'
import { liftNodes, TopicOption } from 'components/types'

type LoadOptionsProps = {
  viewerId: string,
  linkId: string,
  selectedRepoId: string,
}

function makeLoadOptions({ viewerId, linkId, selectedRepoId }: LoadOptionsProps) {
  const environment = useRelayEnvironment()

  return useCallback((searchString: string) => {
    return new Promise<readonly TopicOption[]>((resolve) => {
      const variables = { viewerId, linkId, searchString, selectedRepoId }
      fetchQuery<RefetchQueryType>(environment, RefetchQuery, variables)
        .subscribe({
          next(data) {
            const options =
              makeOptions(data.view.link?.repoLink?.availableParentTopics.synonyms || [])
            resolve(options)
          },
        })
    })
  }, [makeOptions, viewerId, linkId, selectedRepoId])
}

const repoLinkFragment = graphql`
  fragment RepoLinkParentTopics_repoLink on RepoLink {
    linkId

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
  repoLink: RepoLinkParentTopics_repoLink$key,
  selectedRepoId: string,
  viewerId: string,
}

export default function EditRepoLinkParentTopics({ selectedRepoId, viewerId, ...rest }: Props) {
  const repoLink = useFragment(repoLinkFragment, rest.repoLink)
  const selectedTopics = makeOptions(liftNodes(repoLink.selectedTopics))
  const linkId = repoLink.linkId
  const updateTopics = makeUpdateLinkParentTopicsCallback({ linkId, selectedRepoId })
  const loadOptions = makeLoadOptions({ linkId, selectedRepoId, viewerId })

  return (
    <EditParentTopicList
      loadOptions={loadOptions}
      selectedTopics={selectedTopics}
      updateTopics={updateTopics}
    />
  )
}
