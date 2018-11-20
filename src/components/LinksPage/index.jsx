// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import type { OrganizationType } from '../types'
import { liftNodes } from '../../utils'
import Link from '../ui/Link'
import List from '../ui/List'

type Props = {
  organization: OrganizationType,
}

const LinksPage = ({ organization, ...props }: Props) => {
  const links = liftNodes(organization.links)
  return (
    <div>
      <List
        placeholder="There are no links"
        hasItems={!isEmpty(links)}
      >
        { links.map(link => (
          <Link
            key={link.id}
            organization={organization}
            link={link}
            {...props}
          />
        )) }
      </List>
    </div>
  )
}

export const query = graphql`
  query LinksPage_query_Query($orgId: ID!) {
    organization(id: $orgId) {
      ...LinksPage_organization
    }
  }
`

export default createFragmentContainer(LinksPage, graphql`
  fragment LinksPage_organization on Organization {
    ...Link_organization

    links(first: 1000) @connection(key: "Organization_links") {
      edges {
        node {
          id
          ...Link_link
        }
      }
    }
  }
`)
