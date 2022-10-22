import React from 'react'
import { screen, waitFor } from '@testing-library/react'
import { renderWithUser } from 'components/test-utils'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import graphql from 'babel-plugin-relay/macro'
import { EditLinkTestQuery } from '__generated__/EditLinkTestQuery.graphql'
import EditLink from '.'

jest.mock('found',
  () => ({
    Link: ({ to, children }: any) => {
      return `<Link href="${to.pathname}">${children}</a>`
    },
  }),
)

const testQuery = graphql`
  query EditLinkTestQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $linkId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...EditLink_viewer
      }

      link(id: $linkId) {
        ...EditLink_link
      }
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<EditLinkTestQuery>(testQuery,
    { linkId: 'link-id', viewerId: 'viewer-id' })

  return (
    <EditLink
      link={data.view!.link!}
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
        Link() {
          return {
            repoLinks: [
              { repo: { id: 'repo-1' }, viewerCanUpdate: true },
              { repo: { id: 'repo-2' }, viewerCanUpdate: true },
            ],
          }
        },

        User() {
          return {
            selectedRepoId: 'repo-2',
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

describe('<EditLink>', () => {
  it('renders', async () => {
    await setup()
    expect(screen.getByTestId('edit-link')).toBeInTheDocument()
  })

  it('only shows an edit form if the repo is selected', async () => {
    await setup()
    const matchingType = (repoLinkId: string, type: string) =>
      screen.getByTestId(repoLinkId).getElementsByClassName(type)

    expect(matchingType('repo-link-repo-1', 'view-repo-link')).toHaveLength(1)
    expect(matchingType('repo-link-repo-2', 'edit-repo-link')).toHaveLength(1)
  })
})
