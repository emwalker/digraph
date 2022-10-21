import React, { FormEvent, useCallback } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'

import { EdgeTypeOf } from 'components/types'
import selectRepositoryQuery from 'mutations/selectRepositoryMutation'
import { selectRepositoryMutation } from '__generated__/selectRepositoryMutation.graphql'
import {
  SelectRepository_viewer$key,
  SelectRepository_viewer$data as ViewerType,
} from '__generated__/SelectRepository_viewer.graphql'

type EdgeType = EdgeTypeOf<ViewerType['repos']>

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

const viewerFragment = graphql`
  fragment SelectRepository_viewer on User {
    selectedRepo {
      id
      isPrivate
    }

    repos(first: 100) {
      edges {
        isSelected

        node {
          fullName
          id
        }
      }
    }
  }
`

export default function SelectRepository(props: Props) {
  const selectRepo = useMutation<selectRepositoryMutation>(selectRepositoryQuery)[0]
  const viewer = useFragment(viewerFragment, props.viewer)

  const onChange = useCallback((event: FormEvent<HTMLSelectElement>) => {
    const repoId = event.currentTarget.value === '' ? null : event.currentTarget.value
    selectRepo({
      variables: { input: { repositoryId: repoId } },
    })
  }, [selectRepo])

  const repositoryEdges = viewer.repos?.edges || []
  const selectedId = viewer.selectedRepo?.id || undefined

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
          <option key="0" value="">Select a repository</option>
          {repositoryEdges.map(renderOption)}
        </select>
      </dd>
    </dl>
  )
}