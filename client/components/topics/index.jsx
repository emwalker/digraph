// @flow
import React from 'react'

import Layout from '../Layout'

type Props = {
  viewer: {
    name: string,
  }
}

export default ({ viewer: { name } }: Props) => (
  <Layout>
    <h1>Topics</h1>
    <p className="lead">
      List of topics visible to { name }
    </p>
  </Layout>
)
