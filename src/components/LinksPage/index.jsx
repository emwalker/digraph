// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import ItemList from '../ui/ItemList'
import { liftNodes } from '../../utils'

type Props = {
  organization: {
    links: Object,
  },
  relay: {
    environment: Object,
  },
  viewer: Object,
}

const LinksPage = ({ organization: { links }, ...props }: Props) => (
  <div>
    <div className="Subhead">
      <div className="Subhead-heading">Links</div>
    </div>
    <ItemList
      title="Links"
      items={liftNodes(links)}
      {...props}
    />
  </div>
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
  }

  fragment LinksPage_organization on Organization {
    id
    resourceId

    links(first: 100) @connection(key: "Organization_links") {
      edges {
        node {
          display: title
          resourcePath: url

          topics {
            edges {
              node {
                name
                resourcePath
              }
            }
          }
        }
      }
    }
  }
`)
