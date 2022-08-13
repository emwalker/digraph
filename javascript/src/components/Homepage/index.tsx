import React from 'react'
import { graphql } from 'react-relay'
import { Router } from 'found'

import Page from 'components/ui/Page'
import useDocumentTitle from 'utils/useDocumentTitle'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import { Homepage_homepage_QueryResponse as Response } from '__generated__/Homepage_homepage_Query.graphql'
import LineItem from './LineItem'
import SearchBox from './SearchBox'

type ViewType = Response['view']

type Props = {
  router: Router,
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

  let stats = view.stats

  return (
    <div className="f4">
      <div className="homepageHero">
        <div className="homepageBackground" />

        <div className="homepageContent">
          <a className="homepageLogo" href="/">
            <div className="homepageAppName">
              Digraph
            </div>

            <div className="mb-3">
              <DigraphLogo height="60px" width="60px" fill="#fff" />
            </div>
          </a>
          <p className="homepageSubtitle">
            Organize the world
          </p>
        </div>
      </div>

      <Page>
        <p className="homepageDescription">
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
            {` ${stats.linkCount?.toLocaleString() || 0} `}
            links and
            {` ${stats.topicCount?.toLocaleString() || 0} `}
            topics.
          </div>
        </div>

        <SearchBox className="homepageSearch" router={router} />
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
    stats {
      linkCount
      topicCount
    }

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
