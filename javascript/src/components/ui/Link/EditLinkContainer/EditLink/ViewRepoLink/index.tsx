import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { ViewRepoLink_repoLink$key } from '__generated__/ViewRepoLink_repoLink.graphql'
import { borderColor } from 'components/helpers'

type Props = {
  repoLink: ViewRepoLink_repoLink$key,
}

const repoLinkFragment = graphql`
  fragment ViewRepoLink_repoLink on RepoLink {
    displayColor
    details {
      title
    }
  }
`

export default function ViewRepoLink(props: Props) {
  const repoLink = useFragment(repoLinkFragment, props.repoLink)
  const details = repoLink.details

  return (
    <li className="Box-row" style={{ borderColor: borderColor(repoLink.displayColor) }}>
      <div>{ details?.title || '(inherited title)' } </div>
    </li >
  )
}
