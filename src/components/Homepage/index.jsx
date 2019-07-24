import React from 'react'
import { graphql } from 'react-relay'
import classNames from 'classnames'

import { Homepage_homepage_QueryResponse as Response } from './__generated__/Homepage_homepage_Query.graphql'
import LineItem from './LineItem'
import SearchBox from './SearchBox'
import styles from './styles.module.css'

type ViewType = $PropertyType<Response, 'view'>

type Props = {
  router: Object,
  view: ViewType,
}

const Homepage = ({ view, router }: Props) => (
  <div
    className={classNames(styles.container, 'px-3 px-md-6 px-lg-0')}
  >
    <h2 className="mb-2">
      Digraph
    </h2>

    <ul className={classNames(styles.list, 'ml-4 f4')}>
      <li>Save links in a mind map–like network of topics.</li>
      <li>Keep track of everything you&apos;ve read or might want to read in the future.</li>
      <li>
        Gain control over your reading and turn the flood of information into knowledge.
      </li>
    </ul>

    <h4>Recent updates</h4>
    <div className="f4">
      {view.activity.edges.map(({ node }) => <LineItem key={node.description} item={node} />)}

      <div>
        There are currently
        {` ${view.linkCount.toLocaleString()} `}
        links and
        {` ${view.topicCount.toLocaleString()} `}
        topics.
      </div>
    </div>

    <SearchBox className={styles.search} router={router} />
  </div>
)

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
