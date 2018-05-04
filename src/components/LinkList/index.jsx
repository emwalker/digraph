// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import AddLink from './AddLink'
import ListView from '../ui/ListView'
import { liftNodes } from '../../utils'

type Props = {
  links: Object,
  organization: Object,
  relay: {
    environment: Object,
  },
  title: string,
  viewer: Object,
}

const LinkList = ({
  title, links, organization, relay, viewer,
}: Props) => (
  <ListView
    title={title}
    items={liftNodes(links)}
  >
    <AddLink
      className="test-add-link"
      organization={organization}
      relay={relay}
      viewer={viewer}
    />
  </ListView>
)

export default createFragmentContainer(LinkList, graphql`
  fragment LinkList_viewer on User {
    ...AddLink_viewer
  }

  fragment LinkList_organization on Organization {
    ...AddLink_organization
  }

  fragment LinkList_links on LinkConnection {
    edges {
      node {
        id
        display: title
        resourcePath

        topics(first: 5) {
          edges {
            node {
              name
              resourceId
              resourcePath
            }
          }
        }
      }
    }
  }
`)
