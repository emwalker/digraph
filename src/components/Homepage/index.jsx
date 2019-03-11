// @flow
import React, { Component } from 'react'
import { graphql } from 'react-relay'
import { ForceGraph3D } from 'react-force-graph'

type Props = {
  view: {
    topicGraph: string,
  }
}

const nodeLabel = d =>
  `<div style="text-align: center; padding: 8px; color: black; border-radius: 5px; background:
    rgba(239, 243, 255, 0.80); font-size: 28">${d.name}</div>`

const onNodeClick = (node) => {
  const url = `/wiki/topics/${node.id}`
  try {
    window.open(url)
  } catch (e) {
    window.location.href = url
  }
}

type State = {
  height: number,
  width: number,
}

class Homepage extends Component<Props, State> {
  state = {
    height: 150,
    width: 150,
  }

  componentDidMount = () => {
    if (this.graphRef) {
      this.graphRef.d3Force('charge').strength(-100)
    }

    if (this.containerRef) {
      const rect = this.containerRef.getBoundingClientRect()
      this.setState({ height: rect.height, width: rect.width })
    }
  }

  get graphData(): ?string {
    if (!this.props.view)
      return null
    if (!this.cachedGraphData)
      this.cachedGraphData = JSON.parse(this.props.view.topicGraph)

    return this.cachedGraphData
  }

  cachedGraphData: ?string = null
  containerRef: React$ElementRef<*> | null
  graphRef: ?any

  render = () => (
    <div
      ref={(ref) => { this.containerRef = ref }}
      className="topic-chart"
    >
      <div className="Subhead">
        <div className="Subhead-heading">Topics in the general collection</div>
      </div>
      <p className="mb-3">
        These are the topics in the{' '}
        <a href="/wiki/topics/df63295e-ee02-11e8-9e36-17d56b662bc8">general collection</a>.
        Rotate and zoom in to explore.  Hover over a topic to see the label.  Click on the topic to visit
        its page.
      </p>
      {this.graphData && (
        <ForceGraph3D
          backgroundColor="white"
          dagLevelDistance={70}
          dagMode="td"
          graphData={this.graphData}
          height={this.state.height}
          linkColor={() => 'rgba(49, 83, 160, 0.3)'}
          linkWidth={3}
          nodeColor={() => 'rgb(82, 97, 140)'}
          nodeLabel={nodeLabel}
          nodeOpacity={1}
          nodeResolution={15}
          nodeVal={d => d.topicCount}
          onNodeClick={onNodeClick}
          ref={(ref) => { this.graphRef = ref }}
          showNavInfo={false}
          width={this.state.width}
        />
      )}
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
