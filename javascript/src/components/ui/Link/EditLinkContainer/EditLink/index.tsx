import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { backgroundColor, borderColor } from 'components/helpers'
import { EditLink_link$key } from '__generated__/EditLink_link.graphql'
import EditRepoLink from './EditRepoLink'
import ViewRepoLink from './ViewRepoLink'

type Props = {
  link: EditLink_link$key,
  viewer: any,
}

const linkFragment = graphql`
  fragment EditLink_link on Link {
    displayTitle

    repoLinks {
      repo {
        name
      }

      viewerCanUpdate
      displayColor

      ...EditRepoLink_repoLink
      ...ViewRepoLink_repoLink
    }
  }
`

export default function EditLink({ viewer, ...rest }: Props) {
  const link = useFragment(linkFragment, rest.link)

  return (
    <div className="mt-3">
      {link.repoLinks.map((repoLink, index) => (
        <ul
          key={index}
          className="Box Box--condensed mt-3"
          style={{ borderColor: borderColor(repoLink.displayColor) }}
        >
          <div
            className="Box-header"
            style={{
              backgroundColor: backgroundColor(repoLink.displayColor),
              borderColor: borderColor(repoLink.displayColor),
            }}
          >
            {repoLink.repo.name}
          </div>

          {repoLink.viewerCanUpdate
            ? <EditRepoLink
                repoLink={repoLink}
                viewer={viewer}
              />
            : <ViewRepoLink repoLink={repoLink} />}
        </ul>
      ))}
    </div>
  )
}
