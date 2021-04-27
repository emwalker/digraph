import React, { useState, useCallback, ComponentType } from 'react'
import { graphql, fetchQuery, useRelayEnvironment } from 'react-relay/hooks'

import { NodeTypeOf, liftNodes } from 'components/types'
import {
  TopicSuggestionsQuery as Query,
  TopicSuggestionsQueryResponse as Response,
} from '__generated__/TopicSuggestionsQuery.graphql'

type ViewType = Response['view']
type TopicType = NodeTypeOf<ViewType['topics']>

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

type Props = {
  Suggestions: ComponentType<any>,
  setMentionListOpen: (open: boolean) => void,
}

const TopicSuggestions = ({ Suggestions, setMentionListOpen }: Props) => {
  const environment = useRelayEnvironment()
  const [suggestions, setSuggestions] = useState([] as TopicType[])

  const onSearchChange = useCallback(({ value }) => {
    fetchQuery<Query>(environment, query, { searchString: value })
      .subscribe({
        next: (data) => {
          const mentions = liftNodes<TopicType>(data?.view?.topics).filter(Boolean)
          setSuggestions(mentions as TopicType[])
        },
      })
  }, [fetchQuery, liftNodes, setSuggestions])

  return (
    <Suggestions
      onSearchChange={onSearchChange}
      suggestions={suggestions}
      onOpen={() => setMentionListOpen(true)}
      onClose={() => setMentionListOpen(false)}
    />
  )
}

export default TopicSuggestions
