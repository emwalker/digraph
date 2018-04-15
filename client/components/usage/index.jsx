import React, { Component } from 'react'
import Helmet from 'react-helmet'
import { IndexLink } from 'react-router'
import { usage, todo } from './styles.css'
import { example, p, link } from '../homepage/styles.css'
import { setConfig } from '../../actions'

/* eslint react/prop-types: 0 */

class Usage extends Component {
  /* eslint-disable */
  static onEnter({nextState, replaceState, callback}) {
    fetch('/api/v1/conf').then((r) => {
      return r.json();
    }).then((conf) => {
      callback();
    });
  }
  /* eslint-enable */

  render() {
    return (
      <div className={usage}>
        <Helmet title="Usage" />
        <h2 className={example}>Usage:</h2>
        <div className={p}>
          <span className={todo}>TODO: write an article</span>
          <pre className={todo}>
            config:
            {JSON.stringify(this.props.config, null, 2)}
          </pre>
        </div>
        <br />
        go <IndexLink to="/" className={link}>home</IndexLink>
      </div>
    )
  }
}

export default Usage
