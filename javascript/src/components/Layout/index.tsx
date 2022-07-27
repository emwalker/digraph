import React, { ReactNode } from 'react'
import { graphql } from 'react-relay'
import { Match, Router } from 'found'
import { MediaQueryProvider } from 'react-responsive-hoc'

import { LayoutQueryResponse } from '__generated__/LayoutQuery.graphql'
import FlashMessages from '../FlashMessages'
import Header from './Header'
import Footer from './Footer'

type AlertsType = LayoutQueryResponse['alerts']
type ViewType = LayoutQueryResponse['view']

type Props = {
  alerts: AlertsType,
  children?: ReactNode,
  router: Router,
  match: Match,
  view: ViewType,
}

export const query = graphql`
query LayoutQuery(
  $viewerId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $searchString: String,
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
    searchString: $searchString,
  ) {
    viewer {
      ...DesktopNav_viewer
      ...MobileNav_viewer
    }

    ...SearchBox_view
  }
}`

const Layout = ({ alerts, children, view, match, router }: Props) => (
  <MediaQueryProvider width={1600} height={800}>
    <div className="layoutComponent">
      <div>
        <Header
          location={match.location}
          router={router}
          view={view}
          viewer={view.viewer}
        />
        <div className="clearfix">
          <FlashMessages initialAlerts={alerts} />
          { children }
        </div>
        <Footer />
      </div>
    </div>
  </MediaQueryProvider>
)

export default Layout
