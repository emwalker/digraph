import React, { useState, useCallback, ComponentType } from 'react'
import { graphql, fetchQuery, useRelayEnvironment } from 'react-relay/hooks'
import {
  MentionSuggestionsPubProps,
} from '@draft-js-plugins/mention/lib/MentionSuggestions/MentionSuggestions'

import {
  TopicSuggestionsQuery as Query,
  TopicSuggestionsQuery$data as Response,
} from '__generated__/TopicSuggestionsQuery.graphql'

type Mutable<Type> = {
  -readonly [Key in keyof Type]: Type[Key];
};

type ViewType = Response['view']
type SynonymMatches = Mutable<ViewType['topicLiveSearch']['synonyms']>

const query = graphql`
  query TopicSuggestionsQuery (
    $searchString: String,
  ) {
    view(viewerId: "") {
      topicLiveSearch(searchString: $searchString) {
        synonyms {
          name: displayName
          link: id
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
  const [suggestions, setSuggestions] = useState([] as SynonymMatches)

  const onSearchChange = useCallback(({ value }: { value: string }) => {
    // Workaround for a draft-js-plugins/mention issue
    const modifiedValue = value.replace(/^n:/i, '')
    fetchQuery<Query>(environment, query, { searchString: modifiedValue })
      .subscribe({
        next: (data) => {
          const mentions = data?.view?.topicLiveSearch?.synonyms || []
          setSuggestions(mentions as SynonymMatches)
        },
      })
  }, [fetchQuery, setSuggestions])

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
