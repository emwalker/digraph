import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

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
            id: 'topic-id:repo-id',
            topicId: 'topic-id',
            viewerCanUpdate: true,
            synonyms: [
              { name: 'Global heating', locale: 'en' },
            ],
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
    const addButton = screen.getByTestId('add-button')
    expect(addButton).toBeDisabled()

    await user.type(input, '     ')
    expect(addButton).toBeDisabled()

    await user.type(input, 'synonym')
    expect(addButton).not.toBeDisabled()
  })

  it('adds a new synonym to the bottom of the list', async () => {
    const { environment, user } = await setup()

    let list = screen.getByTestId('synonym-list').innerHTML
    const input = screen.getByTestId('synonym-input')
    const addButton = screen.getByTestId('add-button')

    expect(list).toContain('Global heating')
    expect(list).not.toContain('Climate change')

    await user.type(input, 'Climate change')
    await user.click(addButton)

    await waitFor(() => {
      environment.mock.resolveMostRecentOperation((operation) =>
        MockPayloadGenerator.generate(operation, {
          RepoTopic() {
            return {
              id: 'topic-id:repo-id',
              topicId: 'topic-id',
              repoId: 'repo-id',
              synonyms: [
                { name: 'Global heating', locale: 'en' },
                { name: 'Climate change', locale: 'en' },
              ],
            }
          },
        }),
      )
    })

    list = screen.getByTestId('synonym-list').innerHTML
    expect(list).toContain('Global heating')
    expect(list).toContain('Climate change')
  })
})
