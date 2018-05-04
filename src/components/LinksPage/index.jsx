// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import LinkList from '../LinkList'

type Props = {
  organization: {
    links: Object,
  },
  relay: {
    environment: Object,
  },
  viewer: Object,
}

const LinksPage = ({ organization, ...props }: Props) => (
  <LinkList
    title="Links"
    links={organization.links}
    organization={organization}
    {...props}
  />
)

export const query = graphql`
  query LinksPage_query_Query($organizationId: String!) {
    viewer {
      ...LinksPage_viewer
    }

    organization(resourceId: $organizationId) {
      ...LinksPage_organization
    }
  }
`

export default createFragmentContainer(LinksPage, graphql`
  fragment LinksPage_viewer on User {
    name
    ...LinkList_viewer
  }

  fragment LinksPage_organization on Organization {
    id
    resourceId

    ...LinkList_organization

    links(first: 100) @connection(key: "Organization_links") {
      edges {
        node {
          id
        }
      }
      ...LinkList_links
    }
  }
`)
