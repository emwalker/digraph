// @flow
import React from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

type Props = {
  viewer: {
    name: string,
  }
}

const Homepage = ({ viewer }: Props) => (
  <div>
    <div className="Subhead Subhead--spacious">
      <div className="Subhead-heading">Hello {viewer.name}</div>
    </div>
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
  </div>
)

export const query = graphql`
query Homepage_homepage_Query {
  viewer {
    ...Homepage_viewer
  }
}`

export default createFragmentContainer(Homepage, graphql`
  fragment Homepage_viewer on User {
    name
  }
`)
