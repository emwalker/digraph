'use client'

import { useSuspenseQuery } from '@apollo/experimental-nextjs-app-support/ssr'
import { Anchor, Box, Card, Code, List, Title } from '@mantine/core'
import Link from 'next/link'
import { graphql } from '@/lib/__generated__/gql'
import classes from './index.module.css'
import { SearchResultsQuery } from '@/lib/__generated__/graphql'
import SearchBox from '../SearchBox'

const query = graphql(/* GraphQL */ ` query SearchResults(
  $repoIds: [ID!]!, $topicId: ID!, $searchString: String!, $viewerId: ID!
) {
  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {  
    topic(id: $topicId) {
      displayName
      displaySynonyms {
        name
      }

      children(searchString: "", first: 50) {
        edges {
          node {
            ... on Topic {
              id
              displayName
            }

            ... on Link {
              id
              displayTitle
              displayUrl
            }
          }
        }
      }
    }
  }
}`)

type Connection = NonNullable<SearchResultsQuery['view']['topic']>['children']

function searchResults(conn: Connection) {
  const edges = conn?.edges || []

  return edges.map((edge) => {
    if (edge == null) return null
    const { node } = edge
    if (node == null) return null

    if (node.__typename === 'Topic') {
      const { id, displayName } = node
      return (
        <Card
          key={id}
          component={Link}
          href={`/topics/${id}`}
          padding="sm"
          radius="md"
          className={classes.card}
        >
          {displayName}
        </Card>
      )
    }

    if (node.__typename === 'Link') {
      const { id, displayTitle, displayUrl } = node
      return (
        <Card
          key={id}
          padding="sm"
          radius="md"
          className={classes.card}
        >
          <Anchor href={displayUrl}>{displayTitle}</Anchor>
          <Code>{displayUrl}</Code>
        </Card>
      )
    }

    return null
  })
}

const synonym = ({ name }: { name: string }) =>
  <List.Item className={classes.synonym} key={name}>{name}</List.Item>

type Props = {
  topicId: string,
}

export default function SearchResults({ topicId }: Props) {
  const { data } = useSuspenseQuery(query, {
    variables: { repoIds: [], topicId, searchString: '', viewerId: '' },
  })
  const { view } = data
  if (view == null) return null
  const topic = view?.topic
  if (topic == null) return null
  const { children: results, displayName, displaySynonyms } = topic

  return (
    <Box className={classes.topicDetail}>
      <Box className={classes.searchInput}>
        <SearchBox searchString="" />
      </Box>

      <Box className={classes.titleDiv}>
        <Title className={classes.title} order={2}>{displayName}</Title>
      </Box>

      {displaySynonyms.length > 1 && (
        <Box>
          <List listStyleType="none" className={classes.synonyms}>
            {displaySynonyms.map(synonym)}
          </List>
        </Box>
      )}

      { results && (
        <div className={classes.results}>
        { searchResults(results) }
        </div>
      )}

      {/* <Pagination total={10} value={1} my="sm" /> */}
    </Box>
  )
}
