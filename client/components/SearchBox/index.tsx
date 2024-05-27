import { ComboboxItem, OptionsFilter, TagsInput } from '@mantine/core'
import { IconSearch } from '@tabler/icons-react'
import { useCallback, useState } from 'react'
import { useSuspenseQuery } from '@apollo/client'
import { useDebounce } from 'use-debounce'
import { graphql } from '@/lib/__generated__/gql'

const icon = <IconSearch />

const query = graphql(/* GraphQL */ ` query SearchBox(
  $repoIds: [ID!]!, $searchString: String!, $viewerId: ID!
) {
  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {
    topicLiveSearch(searchString: $searchString) {
      synonyms {
        displayName
        id
      }
    }
  }
}`)

// We don't need to filter the options at this point
const optionsFilter: OptionsFilter = ({ options }) => options

type Props = {
  searchString: string,
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export default function SearchBox({ searchString }: Props) {
  const [searchValue, setSearchValue] = useState('')
  const [debouncedSearchValue] = useDebounce(searchValue, 300)
  const { data, refetch } = useSuspenseQuery(query, {
    variables: { repoIds: [], searchString: debouncedSearchValue, viewerId: '' },
  })
  const [searchTerms, setSearchTerms] = useState<string[]>([])

  const queryForTopics = useCallback(async (newSearchValue: string) => {
    setSearchValue(newSearchValue)
    if (newSearchValue.length > 2) refetch()
  }, [setSearchValue])

  const synonyms = data.view?.topicLiveSearch?.synonyms || []
  const options: ComboboxItem[] = []
  const seen = new Set()

  synonyms.forEach(({ displayName, id }) => {
    if (!seen.has(id)) {
      seen.add(id)
      options.push({ value: id, label: displayName })
    }
  })

  return (
    <TagsInput
      allowDuplicates
      clearable
      data={options}
      disabled={false}
      filter={optionsFilter}
      onChange={setSearchTerms}
      onSearchChange={queryForTopics}
      placeholder="Search"
      radius="xl"
      leftSection={icon}
      searchValue={searchValue}
      value={searchTerms}
    />
  )
}
