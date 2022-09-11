import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { SelectedRepo_viewer$key } from '__generated__/SelectedRepo_viewer.graphql'

type Props = {
  viewer: SelectedRepo_viewer$key,
}

export default function SelectedRepo(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment SelectedRepo_viewer on User {
        selectedRepository {
          displayColor
          fullName
          isPrivate
        }
      }
    `,
    props.viewer,
  )

  const repo = viewer.selectedRepository
  if (!repo || !repo.isPrivate) return null
  const backgroundColor = repo.displayColor as string

  return (
    <div className="SelectedRepo-banner" style={{ backgroundColor }}>
      <h2>{repo.fullName || 'Private repo'}</h2>
    </div>
  )
}
