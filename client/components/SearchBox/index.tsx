import { ComboboxItem, OptionsFilter, TagsInput } from '@mantine/core'
import { IconSearch } from '@tabler/icons-react'
import { useCallback, useState } from 'react'
import { useSuspenseQuery } from '@apollo/client'
import { useDebounce } from 'use-debounce'
import { useParams, useSearchParams, useRouter } from 'next/navigation'
import { graphql } from '@/lib/__generated__/gql'
import { SearchBoxQuery } from '@/lib/__generated__/graphql'
import { buildPath } from './buildPath'
import { searchStringFromParams } from '@/lib/searchStringFromParams'

const icon = <IconSearch />

const query = graphql(/* GraphQL */ ` query SearchBox(
  $repoIds: [ID!]!, $searchString: String!, $topicSynonymSearchString: String!, $viewerId: ID!
) {
  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {
    topicLiveSearch(searchString: $topicSynonymSearchString) {
      synonyms {
        displayName
        id
      }
    }

    queryInfo {
      topics {
        displayName
        id
      }
      phrases
    }
  }
}`)

type QueryInfo = SearchBoxQuery['view']['queryInfo']

const termsFromQueryInfo = ({ topics, phrases }: QueryInfo): string[] =>
  topics.map(({ displayName }) => displayName).concat(phrases.length > 0 ? [phrases.join(' ')] : [])

// We don't need to filter the options at this point
const optionsFilter: OptionsFilter = ({ options }) => options

export default function SearchBox() {
  const router = useRouter()
  const params = useParams()
  const searchParams = useSearchParams()
  const searchString = searchStringFromParams(params, searchParams)

  const [currentTerm, setCurrentTerm] = useState('')
  const [debouncedSearchValue] = useDebounce(currentTerm, 300)
  const { data } = useSuspenseQuery(query, {
    variables: {
      repoIds: [],
      searchString,
      topicSynonymSearchString: debouncedSearchValue,
      viewerId: '',
    },
  })
  const { view: { queryInfo } } = data
  const [searchTerms, setSearchTerms] = useState<string[]>(termsFromQueryInfo(queryInfo))

  const synonyms = data.view?.topicLiveSearch?.synonyms || []
  const options: ComboboxItem[] = []
  const seen = new Set()
  const newQueryInfo: Map<string, string> = new Map()

  synonyms.forEach(({ displayName, id }) => {
    if (!seen.has(id)) {
      seen.add(id)
      options.push({ value: id, label: displayName })
      newQueryInfo.set(displayName, id)
    }
  })

  const searchTermsUpdated = useCallback(async (newSearchTerms: string[]) => {
    setSearchTerms(newSearchTerms)

    // Allow the search box to be cleared without having side effects
    if (newSearchTerms.length > 0) {
      const path = buildPath(newSearchTerms, queryInfo, newQueryInfo)
      router.push(path)
    }
  }, [setSearchTerms, queryInfo, params, searchParams, searchStringFromParams])

  return (
    <TagsInput
      allowDuplicates
      clearable
      data={options}
      disabled={false}
      filter={optionsFilter}
      leftSection={icon}
      onChange={searchTermsUpdated}
      onSearchChange={setCurrentTerm}
      placeholder="Search"
      radius="xl"
      searchValue={currentTerm}
      size="lg"
      value={searchTerms}
    />
  )
}
