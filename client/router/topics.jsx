import React from 'react'
import { graphql } from 'react-relay'

import Topics from 'components/topics'

const query = graphql`
query topicsQuery($orgDatabaseId: String!) {
  viewer {
    name
  }

  organization(databaseId: $orgDatabaseId) {
    topics(first: 100) {
      edges {
        node {
          id
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
      orgDatabaseId: 'f9caec7e-405f-11e8-8a2d-8395ff0f4d77',
    }

    const data = await api.fetchQuery(query, variables)

    return {
      title: 'Topics',
      component: (
        <Topics viewer={data.viewer} />
      ),
    }
  },
}

export default route
