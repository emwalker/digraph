import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import graphql from 'babel-plugin-relay/macro'
import { EditTopicTestQuery } from '__generated__/EditTopicTestQuery.graphql'
import EditTopic from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query EditTopicTestQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...EditTopic_viewer
      }

      topic(id: $topicId) {
        ...EditTopic_topic
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<EditTopicTestQuery>(testQuery,
    { topicId: 'topic-id', viewerId: 'viewer-id' })

  return (
    <EditTopic
      topic={data.view!.topic!}
      viewer={data.view!.viewer!}
    />
  )
}

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        Topic() {
          return {
            id: 'topic-1',
            repoTopics: [
              { repo: { id: 'repo-3' }, viewerCanUpdate: true },
              { repo: { id: 'repo-6' }, viewerCanUpdate: true },
            ],
          }
        },

        RepoTopic(_, generateId) {
          return {
            id: `repo-topic-${generateId()}`,
            viewerCanUpdate: true,
            details: {
              timerange: null,
            },
          }
        },

        Repository(_, generateId) {
          return {
            id: `repo-${generateId()}`,
          }
        },

        User() {
          return {
            selectedRepoId: 'repo-6',
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

describe('<EditTopic>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByTestId('edit-topic')).toBeInTheDocument()
  })

  it('only shows an edit form if the repo is selected', async () => {
    await setup()
    const matchingType = (repotopicId: string, type: string) =>
      screen.getByTestId(repotopicId).getElementsByClassName(type)

    expect(matchingType('repo-topic-repo-3', 'view-repo-topic')).toHaveLength(1)
    expect(matchingType('repo-topic-repo-6', 'edit-repo-topic')).toHaveLength(1)
  })
})
