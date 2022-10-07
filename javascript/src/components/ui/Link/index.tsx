import React, { useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { NodeTypeOf, liftNodes, Color } from 'components/types'
import { Link_link$key, Link_link$data as LinkType } from '__generated__/Link_link.graphql'
import Item from '../Item'
import EditLinkLoader from './EditLinkLoader'

type ParentTopicType = NodeTypeOf<LinkType['displayParentTopics']>

type Props = {
  link: Link_link$key,
  viewerId: string | null,
}

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
  const link = useFragment(linkFragment, props.link)
  const [formIsOpen, setFormIsOpen] = useState(false)

  const toggleForm = useCallback(() => {
    setFormIsOpen(!formIsOpen)
  }, [setFormIsOpen, formIsOpen])

  const parentTopics = liftNodes<ParentTopicType>(link.displayParentTopics)
  const showEditButton = !link.loading && link.viewerCanUpdate
  const repoColors = (link.repoLinks || []).map((repoLink) => repoLink.displayColor as Color)

  return (
    <Item
      canEdit={link.viewerCanUpdate}
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
      {formIsOpen && props.viewerId && (
        <EditLinkLoader
          linkId={link.id}
          viewerId={props.viewerId}
        />
      )}
    </Item>
  )
}
