import React, { FormEvent, useCallback } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import { EdgeTypeOf } from 'components/types'
import selectRepositoryMutation, { Input } from 'mutations/selectRepositoryMutation'
import {
  SelectRepository_viewer$key,
  SelectRepository_viewer$data as ViewerType,
} from '__generated__/SelectRepository_viewer.graphql'

type EdgeType = EdgeTypeOf<ViewerType['repositories']>

type Props = {
  viewer: SelectRepository_viewer$key,
}

const renderOption = (edge: EdgeType) => (
  edge && (
    <option
      key={edge.node.fullName}
      value={edge.node.id || undefined}
    >
      {edge.node.fullName}
    </option>
  )
)

export default function SelectRepository(props: Props) {
  const environment = useRelayEnvironment()

  const viewer = useFragment(
    graphql`
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
    props.viewer,
  )

  const onChange = useCallback((event: FormEvent<HTMLSelectElement>) => {
    const repositoryId = event.currentTarget.value
    const input: Input = {
      repositoryId: repositoryId === 'placeholder' ? null : repositoryId,
    }
    selectRepositoryMutation(environment, input)
  }, [selectRepositoryMutation, environment])

  const repositoryEdges = viewer.repositories?.edges || []
  const selectedId = viewer.selectedRepository?.id || undefined

  return (
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
          defaultValue={selectedId}
          onChange={onChange}
        >
          <option key="0" value="placeholder">Select a repository</option>
          {repositoryEdges.map(renderOption)}
        </select>
      </dd>
    </dl>
  )
}