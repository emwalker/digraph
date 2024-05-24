import React, { useEffect } from 'react'
import { Match } from 'found'
import { graphql, useFragment } from 'react-relay'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import { UserSettings_view$key } from '__generated__/UserSettings_view.graphql'
import userSettingsQuery from './userSettingsQuery'
import Sidenav from './Sidenav'
import Account from './Account'
import Support from './Support'

export const query = userSettingsQuery

type Props = {
  match: Match,
  view: UserSettings_view$key,
}

function UserSettings(props: Props) {
  const view = useFragment(
    graphql`
      fragment UserSettings_view on View {
        viewer {
          isGuest
        }

        ...Account_view
      }
    `,
    props.view,
  )
  const viewer = view.viewer

  useEffect(() => {
    if (!viewer?.isGuest) return
    document.location.replace('/')
  }, [viewer])

  if (viewer?.isGuest) return null

  useDocumentTitle('Settings | Digraph')

  return (
    <Page>
      <Sidenav match={props.match} />
      <Account match={props.match} view={view} />
      <Support match={props.match} />
    </Page>
  )
}

export default ({ view, match }: Props) => (
  view
    ? <UserSettings view={view} match={match} />
    : null
)
