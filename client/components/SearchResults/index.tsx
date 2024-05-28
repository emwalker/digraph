'use client'

import { useSuspenseQuery } from '@apollo/experimental-nextjs-app-support/ssr'
import { Anchor, Box, Card, Code, List, Title, Text } from '@mantine/core'
import Link from 'next/link'
import { useSearchParams } from 'next/navigation'
import { graphql } from '@/lib/__generated__/gql'
import classes from './index.module.css'
import { SearchResultsQuery, Topic } from '@/lib/__generated__/graphql'

const query = graphql(/* GraphQL */ ` query SearchResults(
  $repoIds: [ID!]!, $topicId: ID!, $searchString: String!, $viewerId: ID!
) {
  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {  
    topic(id: $topicId) {
      displayName
      displaySynonyms {
        name
      }

      displayParentTopics(first: 10) {
        edges {
          node {
            id
            displayName
          }
        }
      }

      children(searchString: $searchString, first: 50) {
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

type ResultConnection = NonNullable<SearchResultsQuery['view']['topic']>['children']
type TopicConnection = NonNullable<SearchResultsQuery['view']['topic']>['displayParentTopics']

const parentTopic = ({ displayName, id }: Topic) => (
  <Card
    key={id}
    component={Link}
    href={`/topics/${id}`}
    className={classes.card}
  >
    <Text opacity={0.9} size="sm">{displayName}</Text>
  </Card>
)

const searchResults = (conn: ResultConnection) => {
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

const parentTopicsFor = (conn: TopicConnection | null) => {
  if (conn == null) return []
  const { edges } = conn
  if (edges == null) return []
  return edges.map((edge) => edge ? edge.node : null).filter(Boolean) as Topic[]
}

const synonym = ({ name }: { name: string }) =>
  <List.Item className={classes.synonym} key={name}>{name}</List.Item>

type Props = {
  topicId: string,
}

export default function SearchResults({ topicId }: Props) {
  const params = useSearchParams()
  const searchString = params.get('q') || ''
  const { data } = useSuspenseQuery(query, {
    variables: { repoIds: [], topicId, searchString, viewerId: '' },
  })
  const { view } = data
  if (view == null) return null
  const topic = view?.topic
  if (topic == null) return null
  const { children: results, displaySynonyms, displayParentTopics } = topic
  const parentTopics = parentTopicsFor(displayParentTopics)

  return (
    <Box className={classes.searchResults}>
      <Box className={classes.middleCol}>
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

      <Box className={classes.rightCol}>
        <Title order={5}>Parent topics</Title>
        {parentTopics.length > 0 ?
          parentTopics.map(parentTopic) : (
            <Card>
              <Text size="sm" opacity={0.9}>None</Text>
            </Card>
          )}
      </Box>
    </Box>
  )
}
