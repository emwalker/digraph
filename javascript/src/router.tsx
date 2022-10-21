import React from 'react'
import { Express } from 'express'
import { RouteMatch, Route, RedirectException, createRender, makeRouteConfig } from 'found'
import { queryMiddleware } from 'farce'
import { Store, Action, Middleware, Dispatch, AnyAction } from 'redux'
import { Resolver } from 'found-relay'

import Homepage, { query as homepageQuery } from 'components/Homepage'
import TermsOfUse from 'components/TermsOfUse'
import RecentPage, { query as recentPageQuery } from 'components/RecentPage'
import UserSettings, { query as userSettingsQuery } from 'components/UserSettings'
import { TopicPage, query as topicPageQuery } from 'components/TopicPage'
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
  const { topicId, searchString } = variables

  return {
    topicId,
    sessionId,
    viewerId,
    searchString: searchString || '',
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
      <Route path="topics">
        <Route
          path=":topicId"
        >
          <Route
            Component={TopicPage}
            path=""
            prepareVariables={prepareVariables}
            query={topicPageQuery}
          />
          <Route
            path="recent"
            prepareVariables={prepareVariables}
            query={recentPageQuery}
            render={withErrorBoundary(RecentPage)}
          />
        </Route>
      </Route>
    </Route>,
  )
}

type RenderType = ReturnType<typeof createRender>
export const render: RenderType = createRender({})
