import React, { useState, useCallback, ComponentType } from 'react'
import { graphql, fetchQuery, useRelayEnvironment } from 'react-relay/hooks'
import {
  MentionSuggestionsPubProps,
} from '@draft-js-plugins/mention/lib/MentionSuggestions/MentionSuggestions'

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
  Suggestions: ComponentType<MentionSuggestionsPubProps>,
  setMentionListOpen: (open: boolean) => void,
  isOpen: boolean,
}

const TopicSuggestions = ({ Suggestions, isOpen, setMentionListOpen }: Props) => {
  const environment = useRelayEnvironment()
  const [suggestions, setSuggestions] = useState([] as TopicType[])

  const onSearchChange = useCallback(({ value }: { value: string }) => {
    // Workaround for a draft-js-plugins/mention issue
    const modifiedValue = value.replace(/^n:/i, '')
    fetchQuery<Query>(environment, query, { searchString: modifiedValue })
      .subscribe({
        next: (data) => {
          const mentions = liftNodes<TopicType>(data?.view?.topics).filter(Boolean)
          setSuggestions(mentions as TopicType[])
        },
      })
  }, [fetchQuery, liftNodes, setSuggestions])

  return (
    <Suggestions
      onOpenChange={setMentionListOpen}
      onSearchChange={onSearchChange}
      open={isOpen}
      suggestions={suggestions}
    />
  )
}

export default TopicSuggestions
