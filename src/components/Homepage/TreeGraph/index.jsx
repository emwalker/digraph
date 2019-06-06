// @flow
import React, { Component } from 'react'
import { ForceGraph3D } from 'react-force-graph'

const nodeLabel = d => `<div style="text-align: center; padding: 8px; color: black; border-radius:
    5px; background: rgba(239, 243, 255, 0.80); font-size: 28">${d.name}</div>`

const onNodeClick = (node) => {
  const url = `/wiki/topics/${node.id}`
  try {
    window.open(url)
  } catch (e) {
    window.location.href = url
  }
}

type Props = {
  height: number,
  topicGraph: string,
  width: number,
}

class TreeGraph extends Component<Props> {
  cachedGraphData: ?string = null

  graphRef: ?any

  componentDidMount = () => {
    if (this.graphRef) this.graphRef.d3Force('charge').strength(-100)
  }

  get graphData(): ?string {
    if (!this.props.topicGraph) return null
    if (!this.cachedGraphData) this.cachedGraphData = JSON.parse(this.props.topicGraph)

    return this.cachedGraphData
  }

  render = () => (
    <ForceGraph3D
      backgroundColor="white"
      dagLevelDistance={90}
      dagMode="td"
      graphData={this.graphData}
      height={this.props.height}
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
      width={this.props.width}
    />
  )
}

export default TreeGraph
