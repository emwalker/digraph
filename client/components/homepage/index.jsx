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
    <h1>Hello {name}</h1>
    <p className="lead">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent
      vel erat rutrum, egestas ipsum vitae, aliquam nisl. Nunc sodales
      mollis ex eu ultricies. Donec vestibulum augue in erat tristique,
      eu viverra orci mattis. Praesent ac euismod ligula. Nunc commodo
      nec justo nec lacinia. Phasellus metus dolor, varius sit amet turpis
      et, semper elementum massa. Nam venenatis tempor ante id aliquet.
      Curabitur cursus est a fringilla semper. Aliquam eget urna erat.
      Nullam eget vehicula neque.
    </p>
  </Layout>
)
