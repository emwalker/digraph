import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { useLazyLoadQuery } from 'react-relay'
import { MockEnvironment, MockPayloadGenerator } from 'relay-test-utils'
import { Location } from 'farce'
import graphql from 'babel-plugin-relay/macro'

import { renderWithUser } from 'components/test-utils'
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

type Options = {
  repoIds?: string[],
  selectedRepoId?: string | null,
}

async function setup(
  configureMocks: (environment: MockEnvironment, options?: Options) => () => void,
  options?: Options,
) {
  const { environment, user } = renderWithUser(<TestRenderer />)
  expect(screen.getByText('Loading...')).toBeInTheDocument()
  await waitFor(configureMocks(environment, options))
  await waitFor(() => {
    expect(screen.queryByText('Loading...')).not.toBeInTheDocument()
  })
  return { environment, user }
}

function makeMocks(environment: MockEnvironment, options?: Options) {
  const repoIds = options?.repoIds || ['wiki-repo-id']
  const selectedRepoId = options?.selectedRepoId !== undefined
    ? options?.selectedRepoId
    : 'wiki-repo-id'

  return () => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        Topic(context) {
          return {
            __typename: 'Topic',
            children: childrenFor(context),
            displayName: 'Existing topic',
            displayParentTopics: [],
            displaySynonyms: [],
            id: 'topic-id',
            repoTopics: repoIds.map((repoId) => ({
              repoId,
              repo: {
                name: repoId,
                id: repoId,
              },
            })),
            viewerCanUpdate: true,
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
          const isPrivate = selectedRepoId !== 'wiki-repo-id'
          return {
            id: 'viewer-id',
            selectedRepoId,
            selectedRepo: selectedRepoId == null ? null : {
              id: selectedRepoId,
              isPrivate,
            },
          }
        },
      }),
    )
  }
}

describe('<ViewTopicPage>', () => {
  describe('a simple case', () => {
    it('renders', async () => {
      await setup(makeMocks)
      expect(screen.getAllByText('Existing topic').length).toBeGreaterThan(0)
      expect(screen.getByText('Add subtopic')).toBeInTheDocument()
    })

    it('allows a new topic to be added', async () => {
      const { environment, user } = await setup(makeMocks)
      const topicName = 'New topic'
      const connectionId =
        'client:topic-id:__ViewTopicPage_topic_children_connection(searchString:"")'

      environment.mock.queueOperationResolver((op) => {
        return MockPayloadGenerator.generate(op, {
          ID(_, generateId) {
            return generateId()
          },

          Topic() {
            return {
              displayName: topicName,
            }
          },

          SearchResultConnection() {
            return {
              id: connectionId,
            }
          },
        })
      })

      const nameInput = screen.getByTestId('topic-name-input')
      await user.type(nameInput, `${topicName}{enter}`)

      expect(screen.queryAllByText(/Existing topic/).length).toBeGreaterThan(0)
      // FIXME:
      // expect(screen.queryAllByText(new RegExp(topicName)).length).toBeGreaterThan(0)
      expect(screen.queryAllByText(/random/).length).toBe(0)
    })
  })

  describe('adding a link', () => {
    describe('under a Wiki topic', () => {
      const repoIds = ['wiki-repo-id']

      it('allows adding a link if selectedRepoId: private-repo-id', async () => {
        await setup(makeMocks, { repoIds, selectedRepoId: 'private-repo-id' })
        expect(screen.queryByTestId('select-repo')).toBeInTheDocument()

        expect(screen.queryByTestId('link-url-input')).toBeInTheDocument()
        expect(screen.queryByTestId('upserts-disabled')).not.toBeInTheDocument()
      })
    })

    describe('under a topic restricted to private-repo-id', () => {
      const repoIds = ['private-repo-id']

      it('allows adding a link if selectedRepoId: private-repo-id', async () => {
        await setup(makeMocks, { repoIds, selectedRepoId: 'private-repo-id' })
        expect(screen.queryByTestId('select-repo')).toBeInTheDocument()

        expect(screen.queryByTestId('link-url-input')).toBeInTheDocument()
        expect(screen.queryByTestId('upserts-disabled')).not.toBeInTheDocument()
      })

      it('does not allow adding a link if selectedRepoId: wiki-repo-id', async () => {
        await setup(makeMocks, { repoIds, selectedRepoId: 'wiki-repo-id' })
        expect(screen.queryByTestId('select-repo')).toBeInTheDocument()

        expect(screen.queryByTestId('link-url-input')).not.toBeInTheDocument()
        expect(screen.queryByTestId('upserts-disabled')).toBeInTheDocument()
      })

      it('allows adding a link if selectedRepoId: other-repo-id', async () => {
        await setup(makeMocks, { repoIds, selectedRepoId: 'other-repo-id' })
        expect(screen.queryByTestId('select-repo')).toBeInTheDocument()

        expect(screen.queryByTestId('link-url-input')).toBeInTheDocument()
        expect(screen.queryByTestId('upserts-disabled')).not.toBeInTheDocument()
      })
    })
  })

  describe('when no repo is selected', () => {
    it('does not show "Edit" buttons', async () => {
      await setup(makeMocks, { selectedRepoId: null })
      expect(screen.queryByText('Edit')).not.toBeInTheDocument()
    })

    it('does not show the upserts-disabled blankslate', async () => {
      await setup(makeMocks, { selectedRepoId: null })
      expect(screen.queryByTestId('link-url-input')).not.toBeInTheDocument()
      expect(screen.queryByTestId('upserts-disabled')).not.toBeInTheDocument()
    })
  })
})
