'use client'

import { useSuspenseQuery } from '@apollo/experimental-nextjs-app-support/ssr'
import { Button, Card, Pagination, Title } from '@mantine/core'
import { Page } from '@/components/Page'
import { graphql } from '@/lib/__generated__/gql'
import classes from './index.module.css'
import { TopicsQuery } from '@/lib/__generated__/graphql'

const query = graphql(/* GraphQL */ `query Topics($topicId: ID!) {
  view(repoIds: [""], searchString: "", viewerId: "1234") {
    topic(id: $topicId) {
      children(searchString: "", first: 50) {
        edges {
          node {
            ... on Topic {
              id
              displayName
            }
          }
        }
      }
    }
  }
}`)

type Connection = NonNullable<TopicsQuery['view']['topic']>['children']

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
          component="a"
          href={`/topics/${id}`}
          padding="sm"
          radius="md"
          className={classes.card}>
          {displayName}
        </Card>
      )
    }

    return null
  })
}

type Props = {
  topicId: string,
}

export default function Topics({ topicId }: Props) {
  const { data } = useSuspenseQuery(query, { variables: { topicId } })
  const results = data.view?.topic?.children

  return (
    <Page>
      <div className={classes.top}>
        <Title className={classes.title} order={2}>Topics</Title>

        <Button
          component="a"
          href="/topics/new"
          className={classes.addButton}
          >
            Add
        </Button>
      </div>

      { results && (
        <div className={classes.results}>
        { searchResults(results) }
        </div>
      )}

      <Pagination total={10} value={1} my="sm" />
    </Page>
  )
}
