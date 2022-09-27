import React, { useCallback, useEffect, useState } from 'react'
import {
  Environment, fetchQuery, graphql, loadQuery as loadInitialQuery, PreloadedQuery, useFragment,
  usePreloadedQuery, useQueryLoader, useRelayEnvironment,
} from 'react-relay'

import { liftNodes, TopicOption } from 'components/types'
import EditParentTopicList, { makeOptions } from 'components/ui/EditParentTopicList'
import { makeUpdateLinkParentTopicsCallback } from 'mutations/updateLinkParentTopicsMutation'
import {
  EditRepoLinkParentTopics_repoLink$key,
  EditRepoLinkParentTopics_repoLink$data as RepoLinkType,
} from '__generated__/EditRepoLinkParentTopics_repoLink.graphql'
import {
  EditRepoLinkParentTopicsRefetchQuery,
} from '__generated__/EditRepoLinkParentTopicsRefetchQuery.graphql'

const refetchQuery = graphql`
  query EditRepoLinkParentTopicsRefetchQuery(
      $viewerId: ID!,
      $repoIds: [ID!],
      $linkId: String!,
      $searchString: String!,
      $selectedRepoId: String!,
    ) {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      link(id: $linkId) {
        repoLink(repoId: $selectedRepoId) {
          availableTopics: availableParentTopics(searchString: $searchString) {
            synonymMatches {
              value: id
              label: displayName
            }
          }
        }
      }
    }
  }
`

type MakeLoadOptionsProps = {
  environment: Environment,
  linkId: string,
  queryRef: PreloadedQuery<EditRepoLinkParentTopicsRefetchQuery>,
  repoLink: RepoLinkType,
  selectedRepoId: string,
  setQueryRef: any,
  viewerId: string,
}

function makeLoadOptions({
  environment, linkId, repoLink, selectedRepoId, viewerId, setQueryRef, ...rest
}: MakeLoadOptionsProps) {
  // const [isRefetching, setIsRefetching] = useState(false)

  const refetch = useCallback(({ searchString, onComplete }: {
    searchString: string, onComplete: () => void,
  }) => {
    const [queryRef, loadQuery] = useQueryLoader<EditRepoLinkParentTopicsRefetchQuery>(refetchQuery,
      rest.queryRef)

    // if (isRefetching) return
    // setIsRefetching(true)

    const variables = { linkId, viewerId, repoIds: [], selectedRepoId, searchString }
    loadQuery(variables, { fetchPolicy: 'store-only' })
    setQueryRef(queryRef)
    onComplete()

    // fetchQuery(environment, refetchQuery, variables)
    //   .subscribe({
    //     complete() {
    //       console.log('here! 3')
    //       setIsRefetching(false)
    //       onComplete()
    //       console.log('here! 4')
    //     },

    //     error() {
    //       setIsRefetching(false)
    //     },
    //   })
  }, [fetchQuery, environment])

  return useCallback((searchString: string) => {
    return new Promise<readonly TopicOption[]>((resolve) => {
      if (!rest.queryRef) return

      refetch({
        searchString,

        onComplete() {
          console.log('here! 4')
          const data = usePreloadedQuery<EditRepoLinkParentTopicsRefetchQuery>(
            refetchQuery, rest.queryRef,
          )
          const topics = data.view.link?.repoLink?.availableTopics?.synonymMatches || []
          const topicOpitions = makeOptions(topics)
          resolve(topicOpitions)
        },
      })
    })
  }, [refetch, repoLink])
}

const repoLinkFragment = graphql`
  fragment EditRepoLinkParentTopics_repoLink on RepoLink {
    linkId

    selectedTopics: parentTopics(first: 1000) {
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
  repoLink: EditRepoLinkParentTopics_repoLink$key,
  viewerId: string,
  selectedRepoId: string,
}

export default function ParentTopics({ selectedRepoId, viewerId, ...rest }: Props) {
  const environment = useRelayEnvironment()
  const emptyQueryRef = {} as PreloadedQuery<EditRepoLinkParentTopicsRefetchQuery>
  const [queryRef, setQueryRef] = useState(emptyQueryRef)
  const repoLink = useFragment(repoLinkFragment, rest.repoLink)

  const linkId = repoLink.linkId
  const loadOptions = makeLoadOptions({
    environment, selectedRepoId, repoLink, viewerId, queryRef, setQueryRef, linkId,
  })

  const selectedTopics = makeOptions(liftNodes(repoLink.selectedTopics))
  const updateTopics = makeUpdateLinkParentTopicsCallback({ selectedRepoId, linkId })

  useEffect(() => {
    const newQueryRef = loadInitialQuery<EditRepoLinkParentTopicsRefetchQuery>(
      environment,
      refetchQuery,
      { viewerId, linkId: repoLink.linkId, selectedRepoId, repoIds: [], searchString: '' },
    )
    setQueryRef(newQueryRef)
  }, [setQueryRef, viewerId])

  return (
    <EditParentTopicList
      loadOptions={loadOptions}
      selectedTopics={selectedTopics}
      updateTopics={updateTopics}
    />
  )
}
