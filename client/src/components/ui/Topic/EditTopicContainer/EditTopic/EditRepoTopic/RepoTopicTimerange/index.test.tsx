import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import graphql from 'babel-plugin-relay/macro'
import { RepoTopicTimerangeTestQuery } from '__generated__/RepoTopicTimerangeTestQuery.graphql'
import RepoTopicTimerange from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query RepoTopicTimerangeTestQuery(
    $viewerId: ID!,
    $topicId: ID!,
    $selectedRepoId: ID!,
  ) @relay_test_operation {
    view(viewerId: $viewerId) {
      viewer {
        ...RepoTopicTimerange_viewer
      }

      topic(id: $topicId) {
        repoTopic(repoId: $selectedRepoId) {
          ...RepoTopicTimerange_repoTopic
        }
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<RepoTopicTimerangeTestQuery>(testQuery,
    { topicId: 'topic-id', selectedRepoId: 'repo-id', viewerId: 'viewer-id' })

  return (
    <RepoTopicTimerange
      repoTopic={data.view!.topic!.repoTopic!}
      viewer={data.view.viewer!}
    />
  )
}

async function setup({ timerange }: { timerange: Object | null }) {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        User() {
          return {
            selectedRepoId: 'repo-id',
          }
        },

        RepoTopic() {
          return {
            id: 'topic-id:repo-id',
            topicId: 'topic-id',
          }
        },

        RepoTopicDetails() {
          return {
            timerange,
          }
        },

        Timerange() {
          return timerange
        },
      }),
    )
  })

  await waitFor(() => {
    expect(screen.queryByText('Loading...')).not.toBeInTheDocument()
  })

  return { environment, user }
}

describe('<RepoTopicTimerange>', () => {
  it('renders', async () => {
    await setup({ timerange: null })
    expect(screen.getByText('Occurs in time')).toBeInTheDocument()
  })

  it('shows the timerange form after clicking on the checkbox', async () => {
    const { environment, user } = await setup({ timerange: null })

    expect(screen.queryByTestId('timerange-form')).toBeNull()

    const checkbox = screen.getByTestId('timerange-checkbox')
    await user.click(checkbox)

    await waitFor(() => {
      const timerange = {
        startsAt: (new Date()).toISOString(),
      }

      environment.mock.resolveMostRecentOperation((operation) =>
        MockPayloadGenerator.generate(operation, {
          Timerange() {
            return timerange
          },

          RepoTopic() {
            return {
              id: 'topic-id:repo-id',
              topicId: 'topic-id',
              timerange,
            }
          },
        }),
      )
    })

    // Will fail if the form isn't found
    await waitFor(() => screen.getByTestId('timerange-form'))
  })

  it('hides the timerange form in this case', async () => {
    const timerange = {
      startsAt: (new Date()).toISOString(),
      prefixFormat: 'START_YEAR',
    }

    const { environment, user } = await setup({ timerange })

    expect(screen.queryByTestId('timerange-form')).not.toBeNull()

    const checkbox = screen.getByTestId('timerange-checkbox')
    await user.click(checkbox)

    await waitFor(() => {
      environment.mock.resolveMostRecentOperation((operation) =>
        MockPayloadGenerator.generate(operation, {
          Timerange() {
            return null
          },

          RepoTopic() {
            return {
              id: 'topic-id:repo-id',
            }
          },

          RepoTopicDetails() {
            return {
              timerange: null,
            }
          },
        }),
      )
    })

    // Will fail if the form isn't found
    await waitFor(() => expect(screen.queryByTestId('timerange-form')).toBeNull())
  })
})
