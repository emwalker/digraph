// @flow
import React from 'react'
import type { Node } from 'react'
import { graphql } from 'react-relay'
import { MediaQueryProvider } from 'react-responsive-hoc'

import FlashMessages from '../FlashMessages'
import Header from './Header'
import Footer from './Footer'
import styles from './styles.module.css'
import { type LayoutQueryResponse } from './__generated__/LayoutQuery.graphql'

type AlertsType = $PropertyType<LayoutQueryResponse, 'alerts'>
type ViewType = $PropertyType<LayoutQueryResponse, 'view'>

type Props = {
  alerts: AlertsType,
  children?: Node,
  view: ViewType,
}

export const query = graphql`
query LayoutQuery(
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
    defaultOrganization {
      defaultRepository {
        rootTopic {
          resourcePath
        }
      }
    }

    viewer {
      ...DesktopNav_viewer
      ...MobileNav_viewer
    }
  }
}`

const Layout = ({ alerts, children, view }: Props) => (
  <MediaQueryProvider width={1600} height={800}>
    <div className={styles.layout}>
      <div>
        <Header
          viewer={view.viewer}
          defaultOrganization={view.defaultOrganization}
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

Layout.defaultProps = {
  children: null,
}

export default Layout
