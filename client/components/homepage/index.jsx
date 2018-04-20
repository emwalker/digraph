// @flow
import React from 'react'
import Helmet from 'react-helmet'

type Props = {
  viewer: {
    name: string,
  }
}

const title = 'Digraffe link tracker'

const meta = [
  {
    property: 'og:title',
    content: title,
  },
]

export default ({ viewer: { name } }: Props) => (
  <div>
    <Helmet meta={meta} title={title} />
    <nav className="navbar navbar-expand-lg navbar-light bg-light flex-column flex-md-row bd-navbar">
      <a className="navbar-brand mb-0 h1" href="/">Digraffe</a>
      <ul className="navbar-nav mr-auto">
        <li className="nav-item">
          <a className="nav-link" href="/topics">Topics</a>
        </li>
      </ul>
    </nav>
    <div className="container">
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
    </div>
  </div>
)
