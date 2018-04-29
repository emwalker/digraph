import queryMiddleware from 'farce/lib/queryMiddleware'
import createRender from 'found/lib/createRender'
import makeRouteConfig from 'found/lib/makeRouteConfig'
import Route from 'found/lib/Route'
import { Resolver } from 'found-relay'
import React from 'react'
import { graphql } from 'react-relay'
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

import Homepage from '../components/homepage'
import Topics from '../components/Topics'
import Layout from '../components/Layout'

export const historyMiddlewares = [queryMiddleware]

export function createResolver(fetcher) {
  const environment = new Environment({
    network: Network.create((...args) => fetcher.fetch(...args)),
    store: new Store(new RecordSource()),
  })

  return new Resolver(environment)
}

const TopicsQuery = graphql`
query router_Topics_Query($organizationResourceId: String!) {
  viewer {
    ...Topics_viewer
  }

  organization(resourceId: $organizationResourceId) {
    ...Topics_organization
  }
}
`

const HomepageQuery = graphql`
query router_Homepage_Query {
  viewer {
    ...Homepage_viewer
  }
}
`

export const routeConfig = makeRouteConfig(
  <Route
    Component={Layout}
    query={
      graphql`
      query router_Query {
        viewer {
          name
        }
      }
    `}
  >
    <Route
      path="/"
      Component={Homepage}
      query={HomepageQuery}
    />
    <Route
      path="/topics"
      Component={Topics}
      query={TopicsQuery}
      prepareVariables={params => ({
        ...params,
        status: 'any',
        organizationResourceId: 'organization:tyrell',
      })}
    />
  </Route>,
)

export const render = createRender({})
