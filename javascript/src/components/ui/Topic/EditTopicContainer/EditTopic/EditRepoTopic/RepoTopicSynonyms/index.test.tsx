import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'
import { Location } from 'farce'

import graphql from 'babel-plugin-relay/macro'
import { RepoTopicSynonymsTestQuery } from '__generated__/RepoTopicSynonymsTestQuery.graphql'
import RepoTopicSynonyms from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query RepoTopicSynonymsTestQuery(
    $viewerId: ID!,
    $topicId: ID!,
    $selectedRepoId: ID!,
  ) @relay_test_operation {
    view(viewerId: $viewerId) {
      viewer {
        ...RepoTopicSynonyms_viewer
      }

      topic(id: $topicId) {
        repoTopic(repoId: $selectedRepoId) {
          ...RepoTopicSynonyms_repoTopic
        }
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<RepoTopicSynonymsTestQuery>(testQuery,
    { topicId: 'topic-id', selectedRepoId: 'repo-id', viewerId: 'viewer-id' })

  return (
    <RepoTopicSynonyms
      repoTopic={data.view!.topic!.repoTopic!}
      viewer={data.view.viewer!}
    />
  )
}

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        RepoTopic() {
          return {
            topicId: 'topic-id',
            title: 'Reddit',
            url: 'https://reddit.com',
            viewerCanUpdate: true,
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

describe('<RepoTopicSynonyms>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByText('Names and synonyms')).toBeInTheDocument()
  })

  it('disables the "Add" button if there is no text in the input', async () => {
    const { user } = await setup()

    const input = screen.getByTestId('synonym-input')
    expect(input).toHaveTextContent('')
    const button = screen.getByTestId('add-button')
    expect(button).toBeDisabled()

    await user.type(input, 'synonym')
    expect(button).not.toBeDisabled()
  })
})
