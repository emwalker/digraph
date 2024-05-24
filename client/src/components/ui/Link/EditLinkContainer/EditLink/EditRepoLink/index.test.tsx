import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import graphql from 'babel-plugin-relay/macro'
import { EditRepoLinkTestQuery } from '__generated__/EditRepoLinkTestQuery.graphql'
import EditRepoLink from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query EditRepoLinkTestQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $linkId: ID!,
    $selectedRepoId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...EditRepoLink_viewer
      }

      link(id: $linkId) {
        repoLink(repoId: $selectedRepoId) {
          ...EditRepoLink_repoLink
        }
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<EditRepoLinkTestQuery>(testQuery,
    { linkId: 'link-id', selectedRepoId: 'repo-id', viewerId: 'viewer-id' })

  return (
    <EditRepoLink
      repoLink={data.view!.link!.repoLink!}
      viewer={data.view.viewer!}
    />
  )
}

async function setup() {
  const { environment, user } = renderWithUser(<TestRenderer />)

  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() => {
    const details = {
      title: 'Reddit',
      url: 'https://reddit.com',
    }

    environment.mock.resolveMostRecentOperation((operation) =>
      MockPayloadGenerator.generate(operation, {
        RepoLink() {
          return {
            linkId: 'link-id',
            details,
          }
        },

        RepoLinkDetails() {
          return details
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

describe('<EditRepoLink>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByText('Reddit')).toBeInTheDocument()
    expect(screen.getByText('Page title')).toBeInTheDocument()
  })
})
