import React from 'react'

import Homepage from 'components/homepage'

const route = {
  path: '/',
  async action() {
    // const data = await api.fetchQuery(graphql`
    //   viewer {
    //     name
    //   }
    // `)

    return {
      title: 'Home Page',
      component: (
        <Homepage />
      ),
    }
  },
}

export default [route]
