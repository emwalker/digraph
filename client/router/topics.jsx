import React from 'react'
import { graphql } from 'react-relay'

import Topics from 'components/topics'

const query = graphql`
query topicsQuery($organizationId: String!) {
  viewer {
    name
  }

  organization(resourceIdentifier: $organizationId) {
    topics(first: 100) {
      edges {
        node {
          id
          name
          resourceIdentifier
          description
        }
      }
    }
  }
}
`

const route = {
  path: '/topics',
  async action({ api }) {
    const variables = {
      organizationId: 'organization:tyrell',
    }

    const data = await api.fetchQuery(query, variables)

    return {
      title: 'Topics',
      component: (
        <Topics
          viewer={data.viewer}
          organization={data.organization}
        />
      ),
    }
  },
}

export default route
