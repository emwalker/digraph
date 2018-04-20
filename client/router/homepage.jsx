import React from 'react'
import { graphql } from 'react-relay'

import Homepage from 'components/homepage'

const query = graphql`
query homepageQuery {
  viewer {
    name
  }
}
`

const route = {
  path: '/',
  async action({ api }) {
    const data = await api.fetchQuery(query)

    return {
      title: 'Home Page',
      component: (
        <Homepage viewer={data.viewer} />
      ),
    }
  },
}

export default route
