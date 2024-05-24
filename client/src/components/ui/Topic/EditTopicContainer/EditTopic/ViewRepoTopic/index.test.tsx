import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import graphql from 'babel-plugin-relay/macro'
import { ViewRepoTopicTestQuery } from '__generated__/ViewRepoTopicTestQuery.graphql'
import ViewRepoTopic from '.'

const testQuery = graphql`
  query ViewRepoTopicTestQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: ID!,
    $repoId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      topic(id: $topicId) {
        repoTopic(repoId: $repoId) {
          ...ViewRepoTopic_repoTopic
        }
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<ViewRepoTopicTestQuery>(testQuery,
    { topicId: 'topic-id', viewerId: 'viewer-id', repoId: 'repo-id' })

  return (
    <ViewRepoTopic repoTopic={data.view!.topic!.repoTopic!} />
  )
}

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        RepoTopic(_, generateId) {
          return {
            repoId: `repo-${generateId()}`,
            timerangePrefix: '2022-10',
          }
        },
      }),
    )
  })

  await waitFor(() => {
    expect(screen.queryByText('Loading...')).not.toBeInTheDocument()
  })

  return { environment, user }
}

describe('<ViewRepoTopic>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByTestId('repo-topic-repo-1')).toBeInTheDocument()
  })

  it('shows information about the topic timerange', async () => {
    await setup()
    expect(screen.getByTestId('timerange')).toBeInTheDocument()
    expect(screen.getByTestId('timerange')).toHaveTextContent('2022-10')
  })
})
