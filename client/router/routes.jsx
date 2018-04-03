import React from 'react'
import { Route, IndexRoute } from 'react-router'
import App from 'components/app'
import Homepage from 'components/homepage'
import NotFound from 'components/not-found'

/* eslint react/prop-types: 0 */
/* eslint no-param-reassign: 0 */

/**
 * Returns configured routes for different
 * environments. `w` - wrapper that helps skip
 * data fetching with onEnter hook at first time.
 * @param {Object} - any data for static loaders and first-time-loading marker
 * @returns {Object} - configured routes
 */
export default ({ store, first }) => {
  // Make a closure to skip first request
  function w(loader) {
    return (nextState, replaceState, callback) => {
      if (first.time) {
        first.time = false
        return callback()
      }
      return loader ? loader({
        store, nextState, replaceState, callback,
      }) : callback()
    }
  }

  return (
    <Route path="/" component={App}>
      <IndexRoute component={Homepage} onEnter={w(Homepage.onEnter)} />
      <Route path="*" component={NotFound} onEnter={w(NotFound.onEnter)} />
    </Route>
  )
}
