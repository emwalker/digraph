import queryMiddleware from 'farce/lib/queryMiddleware'
import createRender from 'found/lib/createRender'
import makeRouteConfig from 'found/lib/makeRouteConfig'
import Route from 'found/lib/Route'
import { Resolver } from 'found-relay'
import React from 'react'
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

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

export const historyMiddlewares = [queryMiddleware]

export function createResolver(fetcher) {
  const environment = new Environment({
    network: Network.create((...args) => fetcher.fetch(...args)),
    store: new Store(new RecordSource()),
  })
  return new Resolver(environment)
}

const defaultParams = params => ({
  ...params,
  topicId: defaultRootTopicId,
  orgLogin: defaultOrganizationLogin,
})

/* eslint function-paren-newline: 0 */
export const routeConfig = makeRouteConfig(
  <Route
    Component={Layout}
    path="/"
    query={layoutQuery}
    prepareVariables={(params, { location }) => {
      const { q } = location.query
      return {
        ...params,
        repoIds: [],
        searchString: q,
      }
    }}
  >
    <Route
      Component={Homepage}
      query={homepageQuery}
      path="/"
      prepareVariables={defaultParams}
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
      prepareVariables={defaultParams}
      query={recentPageQuery}
      render={withErrorBoundary(RecentPage)}
    />
    <Route
      path="/review"
      prepareVariables={defaultParams}
      query={reviewPageQuery}
      render={withErrorBoundary(ReviewPage)}
    />
    <Route path=":orgLogin">
      <Route path="topics">
        <Route
          path=":topicId"
          render={renderTopicPage}
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

export const render = createRender({})
