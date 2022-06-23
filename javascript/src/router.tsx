import React from 'react'
import { Express } from 'express'
import { RouteMatch, Route, RedirectException, createRender, makeRouteConfig } from 'found'
import { queryMiddleware } from 'farce'
import { Store, Action, Middleware, Dispatch, AnyAction } from 'redux'
import { Resolver } from 'found-relay'

import { defaultRootTopicId, defaultOrganizationLogin } from 'components/constants'
import Homepage, { query as homepageQuery } from 'components/Homepage'
import TermsOfUse from 'components/TermsOfUse'
import RecentPage, { query as recentPageQuery } from 'components/RecentPage'
import ReviewPage, { query as reviewPageQuery } from 'components/ReviewPage'
import UserSettings, { query as userSettingsQuery } from 'components/UserSettings'
import { query as topicPageQuery } from 'components/TopicPage'
import renderTopicPage from 'components/renderTopicPage'
import { query as topicSearchPageQuery } from 'components/TopicSearchPage'
import Layout, { query as layoutQuery } from 'components/Layout'
import withErrorBoundary from 'components/withErrorBoundary'
import SignInPage from 'components/SignInPage'
import { FetcherBase } from './FetcherBase'
import { FoundRelayVariables } from './types'
import { createEnvironment } from './environment'

type RouteStore = Store<any, Action<any>>

export const historyMiddlewares: Middleware<{}, any, Dispatch<AnyAction>>[] = [queryMiddleware]

export function createResolver(fetcher: FetcherBase) {
  const environment = createEnvironment(fetcher)
  return new Resolver(environment)
}

const prepareVariablesFn = (viewer: Express.User) => (variables: FoundRelayVariables) => {
  const viewerId = viewer?.id || ''
  const sessionId = viewer?.sessionId || ''
  const { orgLogin, topicId } = variables
  const topicPath = topicId ? `/${orgLogin}/${topicId}` : `${defaultOrganizationLogin}/${defaultRootTopicId}`

  return {
    orgLogin: defaultOrganizationLogin,
    ...variables,
    topicPath,
    sessionId,
    viewerId,
  }
}

export const createRouteConfig = (store: RouteStore) => {
  const { viewer } = store.getState()
  const prepareVariables = prepareVariablesFn(viewer)

  return makeRouteConfig(
    <Route
      Component={Layout}
      path="/"
      query={layoutQuery}
      prepareVariables={(params: FoundRelayVariables, { location, router }: RouteMatch) => {
        const { q } = location.query

        return {
          ...prepareVariables(params),
          repoIds: [],
          location,
          router,
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
        render={({ props }) => <SignInPage {...props} />}
        path="/login"
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
      <Route
        path="/settings"
      >
        <Route
          render={() => {
            // eslint-disable-next-line @typescript-eslint/no-throw-literal
            throw new RedirectException('/settings/account')
          }}
        />
        <Route
          path="/account"
          prepareVariables={prepareVariables}
          query={userSettingsQuery}
          render={withErrorBoundary(UserSettings)}
        />
        <Route
          path="/support"
          prepareVariables={prepareVariables}
          query={userSettingsQuery}
          render={withErrorBoundary(UserSettings)}
        />
      </Route>
      <Route
        path="/terms-of-use"
        Component={TermsOfUse}
      />
      <Route path=":orgLogin">
        <Route
          path=":topicId"
        >
          <Route
            path=""
            render={renderTopicPage}
            prepareVariables={prepareVariables}
            getQuery={({ location }: RouteMatch) => (
              location.query.q
                ? topicSearchPageQuery
                : topicPageQuery
            )}
          />
          <Route
            path="recent"
            prepareVariables={prepareVariables}
            query={recentPageQuery}
            render={withErrorBoundary(RecentPage)}
          />
          <Route
            path="review"
            prepareVariables={prepareVariables}
            query={reviewPageQuery}
            render={withErrorBoundary(ReviewPage)}
          />
        </Route>
      </Route>
    </Route>,
  )
}

type RenderType = ReturnType<typeof createRender>
export const render: RenderType = createRender({})
