// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay, Edge, Edges } from 'components/types'
import selectRepositoryMutation, { type Input } from 'mutations/selectRepositoryMutation'
import type { SelectRepository_viewer as Viewer } from './__generated__/SelectRepository_viewer.graphql'

type Repositories = $PropertyType<Viewer, 'repositories'>
type RepositoryEdge = Edge<Edges<Repositories>>

type Props = {
  relay: Relay,
  viewer: Viewer,
}

class SelectRepository extends Component<Props> {
  onChange = (event: SyntheticInputEvent<HTMLInputElement>) => {
    const repositoryId = event ? event.target.value : null
    const input: Input = {
      repositoryId: repositoryId === 'placeholder' ? null : repositoryId,
    }
    selectRepositoryMutation(this.props.relay.environment, input)
  }

  get repositoryEdges(): $ReadOnlyArray<?RepositoryEdge> {
    const { repositories } = this.props.viewer
    return repositories && repositories.edges ? repositories.edges : []
  }

  get selectedId(): ?string {
    const repo = this.props.viewer.selectedRepository
    return repo ? repo.id : null
  }

  renderOption = (edge: ?RepositoryEdge) => (
    edge && (
      <option
        key={edge.node.fullName}
        value={edge.node.id}
      >
        {edge.node.fullName}
      </option>
    )
  )

  render = () => (
    <dl className="form-group">
      <dt>
        <label htmlFor="select-repo">New links and topics added to</label>
      </dt>
      <dd>
        <select
          id="select-repo"
          className="form-select"
          aria-label="Repository"
          style={{ width: '100%' }}
          defaultValue={this.selectedId}
          onChange={this.onChange}
        >
          <option key="0" value="placeholder">Select a repository</option>
          {this.repositoryEdges.map(this.renderOption)}
        </select>
      </dd>
    </dl>
  )
}

export default createFragmentContainer(SelectRepository, {
  viewer: graphql`
    fragment SelectRepository_viewer on User {
      selectedRepository {
        id
        isPrivate
      }

      repositories(first: 100) {
        edges {
          isSelected

          node {
            fullName
            id
          }
        }
      }
    }
  `,
})
