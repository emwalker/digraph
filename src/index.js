// @flow
import http from 'http'

/* eslint no-console: 0, global-require: 0 */
let app = require('./server').default

let currentApp = app

const port = process.env.PORT || 3001
const server = http.createServer(app)

server.listen(port, (error) => {
  if (error) console.log(error)

  console.log('🚀 node server listening on', port)
})

// $FlowFixMe
if (module.hot) {
  console.log('✅  Server-side HMR Enabled!')

  // $FlowFixMe
  module.hot.accept('./server', () => {
    console.log('🔁  HMR Reloading `./server`...')

    try {
      app = require('./server').default
      server.removeListener('request', currentApp)
      server.on('request', app)
      currentApp = app
    } catch (error) {
      console.error(error)
    }
  })
}
