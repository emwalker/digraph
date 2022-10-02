import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'
import { Location } from 'farce'

import graphql from 'babel-plugin-relay/macro'
import { ViewTopicPageTestQuery } from '__generated__/ViewTopicPageTestQuery.graphql'
import ViewTopicPage from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query ViewTopicPageTestQuery(
    $repoIds: [ID!],
    $topicId: ID!,
    $viewerId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...ViewTopicPage_viewer
      }

      topic(id: $topicId) {
        ...ViewTopicPage_topic
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<ViewTopicPageTestQuery>(testQuery,
    { topicId: 'topic-id', viewerId: 'viewer-id' })
  const location = { pathname: '/path', query: {} } as Location

  return (
    <ViewTopicPage
      location={location}
      topic={data.view.topic!}
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
        Topic() {
          return {
            __typename: 'Topic',
            id: 'topic-id',
            displayName: 'Topic name',
            viewerCanUpdate: true,
            displayParentTopics: [],
            displaySynonyms: [],

            children: {
              edges: [],
            },
          }
        },

        User() {
          return {
            id: 'viewer-id',
            selectedRepository: {
              id: 'repo-id',
            },
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

describe('<ViewTopicPage>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByText('Topic name')).toBeInTheDocument()
    expect(screen.getByText('Add subtopic')).toBeInTheDocument()
  })

  it('allows a new topic to be added', async () => {
    const { environment, user } = await setup()
    const topicName = 'New topic'

    environment.mock.queueOperationResolver((op) => {
      return MockPayloadGenerator.generate(op, {
        Topic() {
          return {
            displayName: topicName,
          }
        },
      })
    })

    const nameInput = screen.getByTestId('topic-name-input')
    await user.type(nameInput, `${topicName}{enter}`)

    const str = screen.getByTestId('List').innerHTML
    expect(str).toContain(topicName)
    expect(str).not.toContain('random')
  })

  it('allows a new link to be added', async () => {
    const { environment, user } = await setup()
    const linkUrl = 'http://www.google.com'

    const urlInput = screen.getByTestId('link-url-input')
    await user.type(urlInput, `${linkUrl}{enter}`)

    environment.mock.queueOperationResolver((op) => {
      return MockPayloadGenerator.generate(op, {
        Link() {
          return {
            __typename: 'Link',
            id: 'new-link-id',
            displayUrl: linkUrl,
            displayTitle: 'title',
          }
        },
      })
    })

    expect(environment.mock.getAllOperations().length).toBe(1)

    const list = screen.getByTestId('List').innerHTML
    expect(list).toContain(linkUrl)
    expect(list).not.toContain('random')
  })
})
