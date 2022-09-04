import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import { SelectedRepo_viewer as Viewer } from '__generated__/SelectedRepo_viewer.graphql'

type Props = {
  viewer: Viewer,
}

const SelectedRepo = ({ viewer }: Props) => {
  let { selectedRepository: repo } = viewer

  if (!repo || !repo.isPrivate) return null
  console.log('repo', repo)
  const backgroundColor = repo.displayColor as string

  return (
    <div className="SelectedRepo-banner" style={{ backgroundColor }}>
      <h2>{repo.fullName || 'Private repo'}</h2>
    </div>
  )
}


export const UnwrappedSelectedRepo = SelectedRepo

export default createFragmentContainer(SelectedRepo, {
  viewer: graphql`
    fragment SelectedRepo_viewer on User {
      selectedRepository {
        displayColor
        fullName
        isPrivate
      }
    }
  `,
})
