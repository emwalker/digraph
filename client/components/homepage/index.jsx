// @flow
import React, { Component } from 'react'
import Helmet from 'react-helmet'
import { example } from './styles.css'

type Props = {
  store: Object,
  nextState: Object,
  replaceState: any,
  callback: Function,
}

export default class Homepage extends Component<Props> {
  /*eslint-disable */
  static onEnter({store, nextState, replaceState, callback}: Props) {
    // Load here any data.
    callback(); // this call is important, don't forget it
  }
  /* eslint-enable */

  render() {
    return (
      <div>
        <Helmet
          title="Home page"
          meta={[
            {
              property: 'og:title',
              content: 'Golang Isomorphic React/Hot Reloadable/Redux/Css-Modules Starter Kit',
            },
          ]}
        />
        <h1 className={example}>
          Hot Reloadable <br />
          Golang + React + Redux + Css-Modules
          <br />Isomorphic Starter Kit
        </h1>
        <br />
      </div>
    )
  }
}
