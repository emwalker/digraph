import React, { useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { NodeTypeOf, liftNodes, Color } from 'components/types'
import { Link_link$key, Link_link$data as LinkType } from '__generated__/Link_link.graphql'
import { Link_viewer$key } from '__generated__/Link_viewer.graphql'
import Item from '../Item'
import EditLink from './EditLinkContainer'

type ParentTopicType = NodeTypeOf<LinkType['displayParentTopics']>

type Props = {
  link: Link_link$key,
  viewer: Link_viewer$key,
}

export default function Link(props: Props) {
  const [formIsOpen, setFormIsOpen] = useState(false)

  const viewer = useFragment(
    graphql`
      fragment Link_viewer on User {
        isGuest
      }
    `,
    props.viewer,
  )

  const link = useFragment(
    graphql`
      fragment Link_link on Link {
        displayTitle
        displayUrl
        id
        loading
        newlyAdded
        viewerCanUpdate
        showRepoOwnership

        repoLinks {
          displayColor
        }

        displayParentTopics(first: 100) {
          edges {
            node {
              displayName
              id
            }
          }
        }
      }
    `,
    props.link,
  )
  
  const toggleForm = useCallback(() => {
    setFormIsOpen(!formIsOpen)
  }, [setFormIsOpen])

  const parentTopics = liftNodes<ParentTopicType>(link.displayParentTopics)
  const showEditButton = !link.loading && link.viewerCanUpdate
  const repoColors = link.repoLinks.map((repoLink) => repoLink.displayColor as Color)

  return (
    <Item
      canEdit={!viewer.isGuest}
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
      <EditLink
        isOpen={formIsOpen}
        link={link}
        toggleForm={toggleForm}
      />
    </Item>
  )
}
