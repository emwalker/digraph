import queryMiddleware from 'farce/lib/queryMiddleware'
import createRender from 'found/lib/createRender'
import makeRouteConfig from 'found/lib/makeRouteConfig'
import Route from 'found/lib/Route'
import { Resolver } from 'found-relay'
import React from 'react'

import { defaultRootTopicId, defaultOrganizationLogin } from 'components/constants'
import Homepage, { query as homepageQuery } from 'components/Homepage'
import RecentPage, { query as recentPageQuery } from 'components/RecentPage'
import ReviewPage, { query as reviewPageQuery } from 'components/ReviewPage'
import { query as topicPageQuery } from 'components/TopicPage'
import renderTopicPage from 'components/renderTopicPage'
import { query as topicSearchPageQuery } from 'components/TopicSearchPage'
import Layout, { query as layoutQuery } from 'components/Layout'
import withErrorBoundary from 'components/withErrorBoundary'
import SignInPage from 'components/SignInPage'
import SignUpPage from 'components/SignUpPage'
import { createEnvironment } from './environment'

export const historyMiddlewares = [queryMiddleware]

export function createResolver(fetcher) {
  const environment = createEnvironment(fetcher)
  return new Resolver(environment)
}

const prepareVariablesFn = viewer => (params) => {
  const viewerId = viewer ? viewer.id : ''

  return {
    topicId: defaultRootTopicId,
    orgLogin: defaultOrganizationLogin,
    ...params,
    viewerId,
  }
}

/* eslint function-paren-newline: 0 */
export const createRouteConfig = (store) => {
  const { viewer } = store.getState()
  const prepareVariables = prepareVariablesFn(viewer)

  return makeRouteConfig(
    <Route
      Component={Layout}
      path="/"
      query={layoutQuery}
      prepareVariables={(params, { location }) => {
        const { q } = location.query

        return {
          ...prepareVariables(params),
          repoIds: [],
          searchString: q,
        }
      }}
    >
      <Route
        Component={Homepage}
        query={homepageQuery}
        path="/"
        prepareVariables={prepareVariables}
      />
      <Route
        render={withErrorBoundary(SignInPage)}
        path="/login"
      />
      <Route
        render={withErrorBoundary(SignUpPage)}
        path="/join"
      />
      <Route
        path="/recent"
        prepareVariables={prepareVariables}
        query={recentPageQuery}
        render={withErrorBoundary(RecentPage)}
      />
      <Route
        path="/review"
        prepareVariables={prepareVariables}
        query={reviewPageQuery}
        render={withErrorBoundary(ReviewPage)}
      />
      <Route path=":orgLogin">
        <Route path="topics">
          <Route
            path=":topicId"
            render={renderTopicPage}
            prepareVariables={prepareVariables}
            getQuery={({ location }) => (
              location.query.q
                ? topicSearchPageQuery
                : topicPageQuery
            )}
          />
        </Route>
      </Route>
    </Route>,
  )
}

export const render = createRender({})
