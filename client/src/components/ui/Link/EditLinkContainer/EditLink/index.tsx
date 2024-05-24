import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { backgroundColor, borderColor } from 'components/helpers'
import { EditLink_link$key } from '__generated__/EditLink_link.graphql'
import { EditLink_viewer$key } from '__generated__/EditLink_viewer.graphql'
import EditRepoLink from './EditRepoLink'
import ViewRepoLink from './ViewRepoLink'

type Props = {
  link: EditLink_link$key,
  viewer: EditLink_viewer$key,
}

const viewerFragment = graphql`
  fragment EditLink_viewer on User {
    selectedRepoId
    ...EditRepoLink_viewer
  }
`

const linkFragment = graphql`
  fragment EditLink_link on Link {
    displayTitle

    repoLinks {
      repo {
        name
        id
      }

      viewerCanUpdate
      displayColor

      ...EditRepoLink_repoLink
      ...ViewRepoLink_repoLink
    }
  }
`

export default function EditLink(props: Props) {
  const link = useFragment(linkFragment, props.link)
  const viewer = useFragment(viewerFragment, props.viewer)

  return (
    <div className="mt-3" data-testid="edit-link">
      {link.repoLinks.map((repoLink, index) => {
        const repoId = repoLink.repo.id
        const showEditForm = repoLink.viewerCanUpdate && viewer.selectedRepoId === repoId

        return (
          <ul
            key={index}
            data-testid={`repo-link-${repoId}`}
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

            {showEditForm
              ? <EditRepoLink
                repoLink={repoLink}
                viewer={viewer}
              />
              : <ViewRepoLink repoLink={repoLink} />}
          </ul>
        )
      })}
    </div>
  )
}
