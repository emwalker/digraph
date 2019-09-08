// @flow
import React from 'react'
import type { Node } from 'react'
import { graphql } from 'react-relay'
import { MediaQueryProvider } from 'react-responsive-hoc'

import Header from './Header'
import FlashMessages from '../FlashMessages'
import { footer, layout } from './styles.module.css'
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

/* eslint jsx-a11y/anchor-is-valid: 0 */

const Layout = ({ alerts, children, view }: Props) => (
  <MediaQueryProvider width={1600} height={800}>
    <div className={layout}>
      <div>
        <Header
          viewer={view.viewer}
          defaultOrganization={view.defaultOrganization}
        />
        <div className="container-lg clearfix">
          <FlashMessages initialAlerts={alerts} />
          { children }
        </div>
        <footer className={footer}>
          <div className="container-lg px-3 px-md-6 px-lg-0 my-6 pt-2 border-top">
            <p className="mb-2">
              Software available for use under the MIT
              {' '}
              <a href="https://github.com/emwalker/digraph/blob/master/LICENSE.md">license</a>
              . © Eric Walker.
            </p>
          </div>
        </footer>
      </div>
    </div>
  </MediaQueryProvider>
)

Layout.defaultProps = {
  children: null,
}

export default Layout
