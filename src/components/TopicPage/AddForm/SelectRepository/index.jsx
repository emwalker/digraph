// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import selectRepositoryMutation from 'mutations/selectRepositoryMutation'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = {
  relay: {
    environment: Object,
  },
  viewer: {
    selectedRepository: {
      private: boolean,
    },
    repositories: {
      edges: Array<{
        selected: boolean,
        node: Object,
      }>,
    },
  },
}

class SelectRepository extends Component<Props> {
  onChange = (event) => {
    const repositoryId = event ? event.target.value : null
    selectRepositoryMutation(
      this.props.relay.environment,
      [],
      { repositoryId: repositoryId === 'placeholder' ? null : repositoryId },
    )
  }

  get repositoryEdges(): Object[] {
    return this.props.viewer.repositories.edges
  }

  get selectedId(): ?string {
    const repo = this.props.viewer.selectedRepository
    return repo ? repo.id : null
  }

  renderOption = edge => (
    <option
      key={edge.node.fullName}
      value={edge.node.id}
    >
      {edge.node.fullName}
    </option>
  )

  render = () => (
    <dl className="form-group" style={this.style}>
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

export default createFragmentContainer(SelectRepository, graphql`
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
`)
