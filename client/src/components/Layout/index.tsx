import React, { ReactNode } from 'react'
import { graphql } from 'react-relay'
import { Match, Router } from 'found'

import { LayoutQuery$data as Response } from '__generated__/LayoutQuery.graphql'
import FlashMessages from '../FlashMessages'
import Header from './Header'
import Footer from './Footer'
import SelectedRepo from './SelectedRepo'

type AlertsType = Response['alerts']
type ViewType = Response['view']

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
    $repoIds: [ID!],
    $searchString: String,
  ) {
    alerts {
      id
      text
      type
    }

    view(
      repoIds: $repoIds,
      searchString: $searchString,
      viewerId: $viewerId,
    ) {
      viewer {
        ...DesktopNav_viewer
        ...SelectedRepo_viewer
      }

      ...SearchBox_view
    }
  }
`

export default function Layout({ alerts, children, view, match, router }: Props) {
  return (
    <div className="layoutComponent">
      <Header
        location={match.location}
        router={router}
        view={view}
        viewer={view.viewer}
      />
      <div className="clearfix">
        <SelectedRepo viewer={view.viewer} />
        <FlashMessages initialAlertMessages={alerts} />
        { children }
      </div>
      <Footer />
    </div>
  )
}
