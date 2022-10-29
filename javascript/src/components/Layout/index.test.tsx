import React from 'react'
import { screen, waitFor, act } from '@testing-library/react'
import graphql from 'babel-plugin-relay/macro'
import { useLazyLoadQuery } from 'react-relay'
import { MockPayloadGenerator } from 'relay-test-utils'

import { renderWithUser } from 'components/test-utils'
import { LayoutTestQuery } from '__generated__/LayoutTestQuery.graphql'
import Layout from '.'

jest.mock('found',
  () => ({
    Link: ({ children }: any) => {
      return children
    },
  }),
)

const testQuery = graphql`
  query LayoutTestQuery(
    $repoIds: [ID!],
    $viewerId: ID!,
  ) @relay_test_operation {
    view(
      viewerId: $viewerId,
      repoIds: $repoIds,
    ) {
      viewer {
        ...DesktopNav_viewer
        ...SelectedRepo_viewer
      }

      ...SearchBox_view
    }
  }
`

const TestRenderer = () => {
  const data = useLazyLoadQuery<LayoutTestQuery>(testQuery,
    { viewerId: 'viewer-id' })

  const match = { location: { pathname: '' } }

  return (
    // @ts-ignore
    <Layout alerts={[]} view={data.view!} router={{}} match={match} />
  )
}

async function setup(
) {
  const { environment, user } = renderWithUser(<TestRenderer />)
  expect(screen.getByText('Loading...')).toBeInTheDocument()

  await waitFor(() =>
    environment.mock.resolveMostRecentOperation(MockPayloadGenerator.generate),
  )

  await waitFor(() => {
    expect(screen.queryByText('Loading...')).not.toBeInTheDocument()
  })
  return { environment, user }
}

describe('<Layout>', () => {
  it('shows alerts', async () => {
    await setup()

    expect(screen.queryByTestId('alerts')).not.toBeInTheDocument()

    act(() => {
      window.flashMessages?.addMessage({ id: '1', text: 'Alert from a mutation' })
    })

    expect(screen.queryByTestId('alerts')).toBeInTheDocument()
    expect(screen.queryByText(/Alert from a mutation/)).toBeInTheDocument()
  })
})
