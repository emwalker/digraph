// @flow
import React, { useEffect } from 'react'
import { Link } from 'found'
import { createFragmentContainer, graphql } from 'react-relay'

import Page from 'components/ui/Page'
import type { Relay } from 'components/types'
import useDocumentTitle from 'utils/useDocumentTitle'
import { type UserSettings_view as View } from './__generated__/UserSettings_view.graphql'
import DeleteAccount from './DeleteAccount'

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
  relay: Relay,
  view: View,
}

const UserSettings = ({ relay, view }: RenderProps) => {
  const { viewer } = view

  useEffect(() => {
    if (!viewer.isGuest) return
    document.location.replace('/')
  }, [viewer])

  if (viewer.isGuest) return null

  useDocumentTitle('Account | Digraph')

  return (
    <Page>
      <nav className="menu col-3 float-left" aria-label="Settings">
        <span className="menu-heading" id="menu-heading">Settings</span>
        <Link className="menu-item selected" to="/settings/account">Account</Link>
      </nav>
      <div className="col-9 float-left pl-4">
        <DeleteAccount relay={relay} view={view} />
      </div>
    </Page>
  )
}

const Wrapper = createFragmentContainer(UserSettings, {
  view: graphql`
    fragment UserSettings_view on View {
      viewer {
        id
        isGuest
      }

      ...DeleteAccount_view
    }
  `,
})

type Props = {
  view: View,
}

export default ({ props }: { props: Props }) => (
  // eslint-disable-next-line react/prop-types
  props && props.view
    ? <Wrapper {...props} />
    : null
)
