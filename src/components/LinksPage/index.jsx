// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import { isEmpty } from 'ramda'

import { liftNodes } from '../../utils'
import Link from '../ui/Link'
import List from '../ui/List'

const LinksPage = ({ view, ...props }: Props) => {
  const links = liftNodes(view.links)
  return (
    <List
      placeholder="There are no links"
      hasItems={!isEmpty(links)}
    >
      { links.map(link => (
        <Link
          key={link.id}
          link={link}
          {...props}
        />
      )) }
    </List>
  )
}

export const query = graphql`
  query LinksPage_query_Query($orgIds: [ID!], $searchString: String) {
    view(organizationIds: $orgIds) {
      ...LinksPage_view @arguments(searchString: $searchString)
    }
  }
`

export default createFragmentContainer(LinksPage, graphql`
  fragment LinksPage_view on View @argumentDefinitions(
    searchString: {type: "String", defaultValue: ""},
  ) {
    links(first: 50, searchString: $searchString) @connection(key: "Organization_links") {
      edges {
        node {
          id
          ...Link_link
        }
      }
    }
  }
`)
