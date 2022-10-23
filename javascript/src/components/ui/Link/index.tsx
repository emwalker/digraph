import React, { useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { NodeTypeOf, liftNodes, Color } from 'components/types'
import { Link_link$key, Link_link$data as LinkType } from '__generated__/Link_link.graphql'
import { Link_viewer$key } from '__generated__/Link_viewer.graphql'
import Item from '../Item'
import EditLinkLoader from './EditLinkLoader'

type ParentTopicType = NodeTypeOf<LinkType['displayParentTopics']>

type Props = {
  link: Link_link$key,
  viewer: Link_viewer$key,
}

const viewerFragment = graphql`
  fragment Link_viewer on User {
    id
    selectedRepoId
  }
`

const linkFragment = graphql`
  fragment Link_link on Link {
    displayTitle
    displayUrl
    id
    loading
    newlyAdded
    viewerCanUpdate
    showRepoOwnership

    repoLinks {
      inWikiRepo
      displayColor
    }

    displayParentTopics(first: 100) {
      edges {
        node {
          id
          displayName
        }
      }
    }
  }
`

export default function Link(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const link = useFragment(linkFragment, props.link)
  const [formIsOpen, setFormIsOpen] = useState(false)

  const toggleForm = useCallback(() => {
    setFormIsOpen(!formIsOpen)
  }, [setFormIsOpen, formIsOpen])

  const parentTopics = liftNodes<ParentTopicType>(link.displayParentTopics)
  const showEditButton = !link.loading && link.viewerCanUpdate
  const repoColors = (link.repoLinks || []).map((repoLink) => repoLink.displayColor as Color)
  const canEdit = !!(link.viewerCanUpdate && viewer.selectedRepoId)

  return (
    <Item
      canEdit={canEdit}
      className="Box-row--link"
      formIsOpen={formIsOpen}
      newlyAdded={link.newlyAdded}
      repoColors={repoColors}
      showEditButton={showEditButton}
      showLink
      showRepoOwnership={link.showRepoOwnership}
      title={link.displayTitle}
      toggleForm={toggleForm}
      topics={parentTopics}
      url={link.displayUrl}
    >
      {formIsOpen && viewer.id && (
        <EditLinkLoader
          linkId={link.id}
          viewerId={viewer.id}
        />
      )}
    </Item>
  )
}
