import React, { Component, Suspense } from 'react'
import { graphql } from 'react-relay'

const TreeGraph = React.lazy(() => import('./TreeGraph'))

type Props = {
  view: Object,
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

  componentDidMount = () => {
    if (this.containerRef) {
      const rect = this.containerRef.getBoundingClientRect()
      this.setState({ showChart: true, height: 700, width: rect.width })
    }
  }

  get showChart(): boolean {
    return Boolean(this.props.view.topicGraph) && this.state.showChart
  }

  containerRef: React$ElementRef<*> | null

  render = () => (
    <div
      ref={(ref) => { this.containerRef = ref }}
      className="px-3 px-md-6 px-lg-0 topic-chart"
    >
      <div className="Subhead">
        <div className="Subhead-heading">Topics in the general collection</div>
      </div>
      <p className="mb-3">
        These are the topics in the{' '}
        <a href="/wiki/topics/df63295e-ee02-11e8-9e36-17d56b662bc8">general collection</a>.
        Rotate and zoom in to explore.  Hover over a topic to see the label.  Click on a topic to
        visit its page.
      </p>

      <div className="mb-3 topic-chart-container">
        { this.showChart && (
          <Suspense fallback={<div>Loading graphic ...</div>}>
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
    topicGraph
  }
}`

export default Homepage
