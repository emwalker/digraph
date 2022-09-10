import React, { useEffect } from 'react'
import { Match } from 'found'
import { createFragmentContainer, graphql } from 'react-relay'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import { UserSettings_view$data as ViewType } from '__generated__/UserSettings_view.graphql'
import userSettingsQuery, { ViewType as QueryViewType } from './userSettingsQuery'
import Sidenav from './Sidenav'
import Account from './Account'
import Support from './Support'

export const query = userSettingsQuery

type Props = {
  match: Match,
  view: ViewType,
}

const UserSettings = ({ match, view }: Props) => {
  const { viewer } = view

  useEffect(() => {
    if (!viewer?.isGuest) return
    document.location.replace('/')
  }, [viewer])

  if (viewer?.isGuest) return null

  useDocumentTitle('Settings | Digraph')

  return (
    <Page>
      <Sidenav match={match} />
      <Account
        match={match}
        // @ts-expect-error
        view={view}
      />
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

type RenderProps = {
  view: QueryViewType,
  match: Match,
}

export default ({ view, match }: RenderProps) => (
  view
    // @ts-expect-error
    ? <Wrapper view={view} match={match} />
    : null
)
