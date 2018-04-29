import queryMiddleware from 'farce/lib/queryMiddleware'
import createRender from 'found/lib/createRender'
import makeRouteConfig from 'found/lib/makeRouteConfig'
import Route from 'found/lib/Route'
import { Resolver } from 'found-relay'
import React from 'react'
import { graphql } from 'react-relay'
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

import Homepage from '../components/Homepage'
import TopicsPage from '../components/TopicsPage'
import TopicPage from '../components/TopicPage'
import Layout from '../components/Layout'

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
  return <TopicPage topic={props.organization.topic} />
}

const TopicPageQuery = graphql`
query router_TopicPage_Query(
  $orgResourceId: String!,
  $topicResourceId: String!
) {
  organization(resourceId: $orgResourceId) {
    topic(resourceId: $topicResourceId) {
      ...TopicPage_topic
    }
  }
}`

const TopicsPageQuery = graphql`
query router_TopicsPage_Query($orgResourceId: String!) {
  viewer {
    ...TopicsPage_viewer
  }

  organization(resourceId: $orgResourceId) {
    ...TopicsPage_organization
  }
}`

const HomepageQuery = graphql`
query router_Homepage_Query {
  viewer {
    ...Homepage_viewer
  }
}`

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
  >
    <Route
      Component={Homepage}
      query={HomepageQuery}
    />
    <Route
      path="topics"
      prepareVariables={params => ({
        ...params,
        orgResourceId: 'organization:tyrell',
      })}
    >
      <Route
        Component={TopicsPage}
        query={TopicsPageQuery}
      />
      <Route
        path=":uuid"
        render={renderTopicPage}
        query={TopicPageQuery}
        prepareVariables={({ uuid, ...params }) => ({
          topicResourceId: `topic:${uuid}`,
          ...params,
        })}
      />
    </Route>
  </Route>,
)

export const render = createRender({})
