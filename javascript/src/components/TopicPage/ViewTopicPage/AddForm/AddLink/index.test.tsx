import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'
import graphql from 'babel-plugin-relay/macro'
import { AddLinkTestQuery } from '__generated__/AddLinkTestQuery.graphql'
import AddLink from '.'

const TestRenderer = () => {
  const data = useLazyLoadQuery<AddLinkTestQuery>(
    graphql`
      query AddLinkTestQuery(
        $repoIds: [ID!],
        $topicId: ID!,
        $viewerId: ID!,
        $searchString: String,
      ) @relay_test_operation {
        view(
          viewerId: $viewerId,
          repoIds: $repoIds,
        ) {
          viewer {
            ...AddLink_viewer
          }

          topic(id: $topicId) {
            ...AddLink_parentTopic

            children(first: 1000, searchString: $searchString)
            @connection(key: "ViewTopicPage_topic_children") {
              edges {
                node {
                  __typename

                  ... on Topic {
                    id
                    ...Topic_topic
                  }

                  ... on Link {
                    id
                    ...Link_link
                  }
                }
              }
            }
          }
        }
      }
    `,
    { topicId: 'topic-id', viewerId: 'viewer-id', searchString: '' },
  )
  return <AddLink parentTopic={data.view.topic!} viewer={data.view.viewer!} />
}

const searchResults = [
  {
    __typename: 'Topic',
    id: 'child-1-topic',
  },
  {
    __typename: 'Topic',
    id: 'child-2-link',
  },
]

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        Link() {
          return {
            id: 'link-id',
            url: 'https://www.google.com',
            title: 'Google Search',
          }
        },

        Topic(context) {
          const path = context.path || []
          const children = path.toString() == ['view', 'topic'].toString() ? searchResults : []
          return {
            children,
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

describe('<AddLink>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByText('Add link')).toBeInTheDocument()
  })

  it('calls upsertLinkMutation', async () => {
    const { environment, user } = await setup()
    const urlInput = screen.getByTestId('link-url-input')
    await user.type(urlInput, 'https://www.reddit.com{enter}')
    const operation = environment.mock.getMostRecentOperation()
    expect(operation.fragment.node.name).toEqual('upsertLinkMutation')
    expect(operation.root.variables.input.url).toEqual('https://www.reddit.com')
  })
})
