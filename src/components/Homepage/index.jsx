import React, { Component, Suspense } from 'react'
import { graphql } from 'react-relay'
import { Link } from 'found'

import { toEverything } from 'components/navigation'

const TreeGraph = React.lazy(() => import('./TreeGraph'))

const placeholder = (
  <div className="topic-chart-placeholder">
    <div className="loader" />
  </div>
)

type Props = {
  view: {
    linkCount: number,
    topicCount: number,
    topicGraph: ?string,
  },
}

type State = {
  height: number,
  showChart: boolean,
  width: number,
}

class Homepage extends Component<Props, State> {
  state = {
    showChart: false,
    height: 150,
    width: 150,
  }

  containerRef: React$ElementRef<*> | null

  componentDidMount = () => {
    if (this.containerRef) {
      const rect = this.containerRef.getBoundingClientRect()
      this.setState({ showChart: true, height: 700, width: rect.width })
    }
  }

  get showChart(): boolean {
    return Boolean(this.props.view.topicGraph) && this.state.showChart
  }

  render = () => (
    <div
      ref={(ref) => { this.containerRef = ref }}
      className="px-3 px-md-6 px-lg-0 topic-chart"
    >
      <div className="Subhead">
        <div className="Subhead-heading">Topics in the general collection</div>
      </div>
      <p className="mb-3">
        These are the topics in the
        {' '}
        <Link to={toEverything}>general collection</Link>
        . There are
        {' '}
        {this.props.view.topicCount}
        {' '}
        topics in this collection, categorizing
        {' '}
        {this.props.view.linkCount}
        {' '}
        links between them. Rotate and zoom in to explore.  Hover over
        a topic to see the label. Click on a topic to visit its page.
      </p>

      <div className="mb-3 topic-chart-container">
        { this.showChart && (
          <Suspense fallback={placeholder}>
            <TreeGraph
              height={this.state.height}
              topicGraph={this.props.view.topicGraph}
              width={this.state.width}
            />
          </Suspense>
        )}
      </div>

      <p className="mb-3">
        Many of the topics above have subtopics and links associated with them. Making it easy to
        create and manage a network of topics facilitates keeping track of thousands of links.  Once
        a topic becomes too crowded, it can be cleaned up by moving links into one or more
        subtopics.
      </p>
    </div>
  )
}

export const query = graphql`
query Homepage_homepage_Query(
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
) {
  view(
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    linkCount
    topicCount
    topicGraph
  }
}`

export default Homepage
