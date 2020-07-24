// @flow
import React, { useState, useCallback } from 'react'
import { graphql, fetchQuery, useRelayEnvironment } from 'react-relay/hooks'

import type { TopicSuggestionsQuery } from './__generated__/TopicSuggestionsQuery.graphql'

const query = graphql`
  query TopicSuggestionsQuery (
    $searchString: String,
  ) {
    view(currentOrganizationLogin: "wiki", viewerId: "") {
      topics(first: 10, searchString: $searchString) {
        edges {
          node {
            name
            link: resourcePath
          }
        }
      }
    }
  }
`

type Props = {|
  Wrapped: any,
  setMentionListOpen: Function,
|}

const TopicSuggestions = ({ Wrapped, setMentionListOpen }: Props) => {
  const environment = useRelayEnvironment()
  const [suggestions, setSuggestions] = useState([])

  const onSearchChange = useCallback(({ value }) => {
    fetchQuery<TopicSuggestionsQuery>(environment, query, { searchString: value })
      .subscribe({
        next: (data) => {
          const mentions = data.view.topics.edges.map(({ node }) => node)
          setSuggestions(mentions)
        },
      })
  }, [fetchQuery])

  return (
    <Wrapped
      onSearchChange={onSearchChange}
      suggestions={suggestions}
      onOpen={() => setMentionListOpen(true)}
      onClose={() => setMentionListOpen(false)}
    />
  )
}

export default TopicSuggestions
