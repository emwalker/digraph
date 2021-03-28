// @flow
import React from 'react'
import { graphql } from 'react-relay'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import { type Homepage_homepage_QueryResponse as Response } from './__generated__/Homepage_homepage_Query.graphql'
import LineItem from './LineItem'
import SearchBox from './SearchBox'
import styles from './styles.module.css'

type ViewType = $PropertyType<Response, 'view'>

type Props = {
  router: Object,
  view: ViewType,
}

const noActivity = (
  <div className="my-3 blankslate border">
    <p>No recent activity found.</p>
  </div>
)

const Homepage = ({ view, router }: Props) => {
  useDocumentTitle('Digraph')

  const recents = (view.activity.edges || []).map(
    (edge) => edge && edge.node && <LineItem key={edge.node.description} item={edge.node} />,
  )

  return (
    <div className="f4">
      <div className={styles.hero}>
        <div className={styles.background} />

        <div className={styles.content}>
          <a className={styles.logo} href="/">
            <div className={styles.appName}>
              Digraph
            </div>

            <div className="mb-3">
              <DigraphLogo height="60px" width="60px" fill="#fff" />
            </div>
          </a>
          <p className={styles.subtitle}>
            Organize the world
          </p>
        </div>
      </div>

      <Page>
        <p className={styles.description}>
          Save links in a mind-map-like network of topics.
          Keep track of everything you&rsquo;ve read or might want to read in the future.
          Gain control over your reading and turn the deluge of information into knowledge.
        </p>

        <h3>Recent updates</h3>
        <div>
          {recents.length > 0
            ? recents
            : noActivity}

          <div>
            There are currently
            {` ${view.linkCount.toLocaleString()} `}
            links and
            {` ${view.topicCount.toLocaleString()} `}
            topics.
          </div>
        </div>

        <SearchBox className={styles.search} router={router} />
      </Page>
    </div>
  )
}

export const query = graphql`
query Homepage_homepage_Query(
  $viewerId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
) {
  view(
    viewerId: $viewerId,
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    linkCount
    topicCount

    activity(first: 3) {
      edges {
        node {
          description
          ...LineItem_item
        }
      }
    }
  }
}`

export default Homepage
