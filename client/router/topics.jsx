import React from 'react'
import { graphql } from 'react-relay'

import Topics from 'components/topics'

const query = graphql`
query topicsQuery($organizationId: ID!) {
  viewer {
    name
  }

  organization(id: $organizationId) {
    topics(first: 100) {
      edges {
        node {
          id
          name
          resourcePath
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
      organizationId: 'T3JnYW5pemF0aW9uOmY5Y2FlYzdlLTQwNWYtMTFlOC04YTJkLTgzOTVmZjBmNGQ3Nw==',
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
