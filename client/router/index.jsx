import ReactDOM from 'react-dom'
import { Promise } from 'when'
import Router from 'universal-router'
import createHistory from 'history/createBrowserHistory'

import toString from './toString'
import routes from './routes'
import '../css'

const router = new Router(routes)

function render(location) {
  router.resolve(location).then((result) => {
    ReactDOM.render(
      result.component,
      document.getElementById('app'),
      () => { document.title = result.title },
    )
  })
}

export function run() {
  window.Promise = window.Promise || Promise
  window.self = window
  // eslint-disable-next-line global-require
  require('isomorphic-fetch')

  const history = createHistory()
  history.listen(render)
  render(history.location)
}

// Export it to render on the Golang server, keep the name sync with
// server/react.go
export const renderToString = toString

// Live style reloading
if (module.hot) {
  let c = 0
  module.hot.accept('../css', () => {
    // eslint-disable-next-line global-require
    require('../css')
    const a = document.createElement('a')
    const link = document.querySelector('link[rel="stylesheet"]')
    a.href = link.href
    a.search = `?${c}`
    c += 1
    link.href = a.href
  })
}
