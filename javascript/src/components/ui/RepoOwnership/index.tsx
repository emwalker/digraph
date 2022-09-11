import React from 'react'

import { Color } from 'components/types'

type Props = {
  showRepoOwnership: boolean,
  repoColors: Color[],
}

export default function RepoOwnership(props: Props) {
  if (!props.showRepoOwnership) return null

  const repoColors = props.repoColors
  const width = repoColors.length === 0 ? '100%' : `${100 / repoColors.length}%`

  return (
    <span className="Progress mt-2 RepoOwnership">
      {repoColors.map((color, index) =>
        <span
          key={index}
          className="Progress-item"
          style={{ backgroundColor: color as string, width }}
        />,
      )}
    </span>
  )
}
