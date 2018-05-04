import queryMiddleware from 'farce/lib/queryMiddleware'
import createRender from 'found/lib/createRender'
import makeRouteConfig from 'found/lib/makeRouteConfig'
import Route from 'found/lib/Route'
import { Resolver } from 'found-relay'
import React from 'react'
import { graphql } from 'react-relay'
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

import Homepage, { query as homepageQuery } from './components/Homepage'
import TopicsPage, { query as topicsPageQuery } from './components/TopicsPage'
import TopicPage, { query as topicPageQuery } from './components/TopicPage'
import LinksPage, { query as linksPageQuery } from './components/LinksPage'
import Layout from './components/Layout'

export const historyMiddlewares = [queryMiddleware]

export function createResolver(fetcher) {
  const environment = new Environment({
    network: Network.create((...args) => fetcher.fetch(...args)),
    store: new Store(new RecordSource()),
  })
  return new Resolver(environment)
}

const renderTopicPage = ({ props, error }: any) => {
  if (error)
    return <div>There was a problem.</div>
  if (!props)
    return <div>loading ...</div>
  return (
    <TopicPage
      topic={props.organization.topic}
      {...props}
    />
  )
}

export const routeConfig = makeRouteConfig(
  <Route
    Component={Layout}
    path="/"
    query={
      graphql`
      query router_Query {
        viewer {
          name
        }
      }`
    }
    prepareVariables={params => ({
      ...params,
      organizationId: 'organization:tyrell',
    })}
  >
    <Route
      Component={Homepage}
      query={homepageQuery}
    />
    <Route path="topics">
      <Route
        Component={TopicsPage}
        query={topicsPageQuery}
      />
      <Route
        path=":uuid"
        render={renderTopicPage}
        query={topicPageQuery}
        prepareVariables={({ uuid, ...params }) => ({
          topicId: `topic:${uuid}`,
          ...params,
        })}
      />
    </Route>
    <Route path="links">
      <Route
        Component={LinksPage}
        query={linksPageQuery}
      />
    </Route>
  </Route>,
)

export const render = createRender({})
