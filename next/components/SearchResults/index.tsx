'use client'

import { useSuspenseQuery } from '@apollo/experimental-nextjs-app-support/ssr'
import { Anchor, Box, Card, Code, List, Title, Text } from '@mantine/core'
import Link from 'next/link'
import { useParams, useSearchParams } from 'next/navigation'
import { graphql } from '@/lib/__generated__/gql'
import classes from './index.module.css'
import { SearchResultsQuery } from '@/lib/__generated__/graphql'
import { searchStringFromParams } from '@/lib/searchStringFromParams'

const query = graphql(/* GraphQL */ ` query SearchResults(
  $repoIds: [ID!]!,
  $topicId: ID!,
  $searchString: String!,
  $queryParamSearchString: String!,
  $viewerId: ID!
) {
  view(repoIds: $repoIds, searchString: $searchString, viewerId: $viewerId) {
    queryInfo {
      topics {
        displayName
        id

        displaySynonyms {
          name
        }

        displayParentTopics {
          edges {
            node {
              displayName
              id
            }
          }
        }
      }
    }

    topic(id: $topicId) {
      displayName

      children(searchString: $queryParamSearchString, first: 50) {
        edges {
          node {
            ... on Topic {
              id
              displayName
              displayParentTopics(first: 10) {
                edges {
                  node {
                    displayName
                    id
                  }
                }
              }
            }

            ... on Link {
              id
              displayTitle
              displayUrl
              displayParentTopics(first: 10) {
                edges {
                  node {
                    displayName
                    id
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}`)

type ResultConnection = NonNullable<SearchResultsQuery['view']['topic']>['children']
type TopicConnection = NonNullable<NonNullable<
    NonNullable<
      NonNullable<SearchResultsQuery['view']['topic']>['children']
    >['edges']
  >[0]
>['node']['displayParentTopics']

type Topic = SearchResultsQuery['view']['queryInfo']['topics'][0]

const searchTopic = ({ displayName, id, displaySynonyms, displayParentTopics }: Topic) => {
  const parentTopics = parentTopicsFor(displayParentTopics)

  return (
    <Card
      key={id}
      className={classes.searchTopic}
    >
      <Link className={classes.topicLink} href={`/topics/${id}`}>{displayName}</Link>
      {parentTopicGroup(parentTopics)}

      {displaySynonyms.length > 1 && (
        <Box className={classes.namesAndSynonyms}>
          <Text>Names and synonyms</Text>
          <List className={classes.searchTopicSynonyms}>
            {displaySynonyms.map(({ name }) => <List.Item key={name}>{name}</List.Item>)}
          </List>
        </Box>
      )}
    </Card>
  )
}

const parentTopicGroup = (parentTopics: Topic[]) => (
  <Box className={classes.parentTopics}>
   {parentTopics.map(({ id, displayName }) => (
     <Link key={id} className={classes.parentTopic} href={`/topics/${id}`}>{displayName}</Link>
   ))}
  </Box>
)

const searchResults = (conn: ResultConnection) => {
  const edges = conn?.edges || []

  return edges.map((edge) => {
    if (edge == null) return null
    const { node } = edge
    if (node == null) return null

    if (node.__typename === 'Topic') {
      const { id, displayName, displayParentTopics } = node
      return (
        <Card
          key={id}
          padding="sm"
          radius="md"
          className={classes.card}
        >
          <Link className={classes.topicLink} href={`/topics/${id}`}>{displayName}</Link>
          {parentTopicGroup(parentTopicsFor(displayParentTopics))}
        </Card>
      )
    }

    if (node.__typename === 'Link') {
      const { id, displayTitle, displayUrl, displayParentTopics } = node
      return (
        <Card
          key={id}
          padding="sm"
          radius="md"
          className={classes.card}
        >
          <Anchor className={classes.linkResult} href={displayUrl}>{displayTitle}</Anchor>
          <Code>{displayUrl}</Code>
          {parentTopicGroup(parentTopicsFor(displayParentTopics))}
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

type Props = {
  topicId: string,
}

export default function SearchResults({ topicId }: Props) {
  const params = useParams()
  const searchParams = useSearchParams()
  const searchString = searchStringFromParams(params, searchParams)

  const { data } = useSuspenseQuery(query, {
    variables: {
      repoIds: [],
      topicId,
      searchString,
      queryParamSearchString: searchParams.get('q') || '',
      viewerId: '',
    },
  })
  const { view } = data
  if (view == null) return null

  const { topic, queryInfo } = view
  if (topic == null) return null

  const { children: results } = topic
  const searchTopics = queryInfo.topics

  return (
    <Box className={classes.searchResults}>
      <Box className={classes.middleCol}>
        { results && (
          <div className={classes.results}>
          { searchResults(results) }
          </div>
        )}

        {/* <Pagination total={10} value={1} my="sm" /> */}
      </Box>

      <Box className={classes.rightCol}>
        <Title order={3}>Search topics</Title>
        {searchTopics.length > 0 ?
          searchTopics.map(searchTopic) : (
            <Card>
              <Text size="sm" opacity={0.9}>None</Text>
            </Card>
          )}
      </Box>
    </Box>
  )
}
