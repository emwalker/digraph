// @flow
import React, { useEffect } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import Page from 'components/ui/Page'
import type { Match, Relay } from 'components/types'
import useDocumentTitle from 'utils/useDocumentTitle'
import { type UserSettings_view as View } from './__generated__/UserSettings_view.graphql'
import Sidenav from './Sidenav'
import Account from './Account'
import Support from './Support'

export const query = graphql`
  query UserSettings_query_Query(
    $viewerId: ID!,
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
  ) {
    alerts {
      id
      text
      type
    }

    view(
      viewerId: $viewerId,
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      ...UserSettings_view
    }
  }
`

type RenderProps = {
  match: Match,
  relay: Relay,
  view: View,
}

const UserSettings = ({ match, relay, view }: RenderProps) => {
  const { viewer } = view

  useEffect(() => {
    if (!viewer.isGuest) return
    document.location.replace('/')
  }, [viewer])

  if (viewer.isGuest) return null

  useDocumentTitle('Settings | Digraph')

  return (
    <Page>
      <Sidenav match={match} />
      <Account match={match} relay={relay} view={view} />
      <Support match={match} />
    </Page>
  )
}

const Wrapper = createFragmentContainer(UserSettings, {
  view: graphql`
    fragment UserSettings_view on View {
      viewer {
        isGuest
      }

      ...Account_view
    }
  `,
})

type Props = {
  props: {
    view: View,
  },
  match: Match,
}

export default ({ props, match }: Props) => (
  // eslint-disable-next-line react/prop-types
  props && props.view
    ? <Wrapper {...props} match={match} />
    : null
)
