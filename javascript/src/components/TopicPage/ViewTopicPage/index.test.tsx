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
    Link: ({ children }: any) => {
      return children
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

function childrenFor(context: MockPayloadGenerator.MockResolverContext) {
  const path = (context.path || []).toString()
  if (path !== ['view', 'topic'].toString()) return {}

  return {
    edges: [
      {
        node: {
          __typename: 'Topic',
          id: 'child-1-topic',
          displayName: 'Climate change',
        },
      },
      {
        node: {
          __typename: 'Link',
          id: 'child-2-link',
          displayTitle: 'Climate change',
          displayUrl: 'https://en.wikipedia.org/wiki/Climate_change',
        },
      },
    ],
  }
}

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
        Topic(context) {
          return {
            __typename: 'Topic',
            id: 'topic-id',
            displayName: 'Existing topic',
            viewerCanUpdate: true,
            displayParentTopics: [],
            displaySynonyms: [],
            children: childrenFor(context),
          }
        },

        Link() {
          return {
            __typename: 'Link',
            id: 'link-id',
            displayTitle: 'Existing link',
            displayUrl: 'https://www.reddit.com',
            viewerCanUpdate: true,
            displayParentTopics: [],
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
    expect(screen.getAllByText('Existing topic').length).toBeGreaterThan(0)
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

  describe('adding a link', () => {
    const linkUrl = 'http://www.google.com'

    beforeEach(async () => {
      const { environment, user } = await setup()

      const urlInput = screen.getByTestId('link-url-input')
      await user.type(urlInput, `${linkUrl}{enter}`)

      environment.mock.queueOperationResolver(MockPayloadGenerator.generate)
      expect(environment.mock.getAllOperations().length).toBe(1)
      expect(screen.getByText('Existing link')).toBeInTheDocument()
    })

    it('works', () => {
      const list = screen.getByTestId('List')
      expect(list).toHaveTextContent(linkUrl)
      expect(list).not.toHaveTextContent('random')
      expect(list).not.toHaveTextContent('Close')
    })

    it('adds the link above existing links and below existing topics', async () => {
      const list = screen.getByTestId('List')
      const titles = Array.from(list.querySelectorAll('li div.item-title'))
        .map((child) => child.textContent)

      expect(titles).toEqual([
        'Existing topic',
        'Fetching link title ...',
        'Existing link',
      ])
    })
  })
})
