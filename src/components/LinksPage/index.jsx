// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import AddLink from './AddLink'
import ListView from '../ui/ListView'
import { liftNodes } from '../../utils'

type Props = {
  organization: {
    links: Object,
  },
  relay: {
    environment: Object,
  }
}

const LinksPage = ({ organization, relay }: Props) => (
  <ListView
    title="Links"
    items={liftNodes(organization.links)}
  >
    <AddLink
      className="test-add-link"
      organization={organization}
      relay={relay}
    />
  </ListView>
)

export const query = graphql`
query LinksPage_query_Query($orgResourceId: String!) {
  viewer {
    ...LinksPage_viewer
  }

  organization(resourceId: $orgResourceId) {
    ...LinksPage_organization
  }
}`

export default createFragmentContainer(LinksPage, graphql`
  fragment LinksPage_viewer on User {
    name
  }

  fragment LinksPage_organization on Organization {
    id
    resourceId

    links(first: 100) @connection(key: "Organization_links") {
      edges {
        node {
          id
          display: title
          resourcePath
        }
      }
    }
  }
`)
