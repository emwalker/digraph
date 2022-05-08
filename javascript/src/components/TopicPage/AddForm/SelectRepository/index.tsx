import React, { Component, FormEvent } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import { EdgeTypeOf } from 'components/types'
import selectRepositoryMutation, { Input } from 'mutations/selectRepositoryMutation'
import {
  SelectRepository_viewer as ViewerType,
} from '__generated__/SelectRepository_viewer.graphql'

type EdgeType = EdgeTypeOf<ViewerType['repositories']>

type Props = {
  relay: RelayProp,
  viewer: ViewerType,
}

class SelectRepository extends Component<Props> {
  onChange = (event: FormEvent<HTMLSelectElement>) => {
    const repositoryId = event.currentTarget.value
    const input: Input = {
      repositoryId: repositoryId === 'placeholder' ? null : repositoryId,
    }
    selectRepositoryMutation(this.props.relay.environment, input)
  }

  get repositoryEdges() {
    const { repositories } = this.props.viewer
    return repositories?.edges || []
  }

  get selectedId() {
    const repo = this.props.viewer.selectedRepository
    return repo ? repo.id : null
  }

  renderOption = (edge: EdgeType) => (
    edge && (
      <option
        key={edge.node.fullName}
        value={edge.node.id || undefined}
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
          defaultValue={this.selectedId || undefined}
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
