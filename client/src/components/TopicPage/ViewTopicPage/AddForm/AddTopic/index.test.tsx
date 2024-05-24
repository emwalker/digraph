import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'
import graphql from 'babel-plugin-relay/macro'
import { AddTopicTestQuery } from '__generated__/AddTopicTestQuery.graphql'
import AddTopic from '.'

const TestRenderer = () => {
  const data = useLazyLoadQuery<AddTopicTestQuery>(
    graphql`
      query AddTopicTestQuery(
        $repoIds: [ID!],
        $topicId: ID!,
        $viewerId: ID!,
      ) @relay_test_operation {
        view(
          viewerId: $viewerId,
          repoIds: $repoIds,
        ) {
          viewer {
            ...AddTopic_viewer
          }

          topic(id: $topicId) {
            ...AddTopic_parentTopic
          }
        }
      }
    `,
    { topicId: 'topic-id', viewerId: 'viewer-id' },
  )
  return <AddTopic parentTopic={data.view.topic!} viewer={data.view.viewer!} />
}

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        Topic() {
          return {
            id: 'topic-id',
            name: 'Topic name',
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

describe('<AddTopic>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByText('Add subtopic')).toBeInTheDocument()
  })

  it('calls upsertTopicMutation', async () => {
    const { environment, user } = await setup()
    const nameInput = screen.getByTestId('topic-name-input')
    await user.type(nameInput, 'New topic{enter}')
    const operation = environment.mock.getMostRecentOperation()
    expect(operation.fragment.node.name).toEqual('upsertTopicMutation')
    expect(operation.root.variables.input.name).toEqual('New topic')
  })
})
