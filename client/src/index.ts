import http from 'http'

let app = require('./server').default

let currentApp = app

const port = process.env.PORT || 3001
const server = http.createServer(app)

server.listen(port, () => {
  console.log('🚀 node server listening on', port)
})

if (module.hot) {
  console.log('✅  Server-side HMR Enabled!')

  module.hot.accept('./server', () => {
    console.log('🔁  HMR Reloading `./server`...')

    try {
      app = require('./server').default
      server.removeListener('request', currentApp)
      server.on('request', app)
      currentApp = app
    } catch (error) {
      console.error('There was a problem starting the server: ', error)
    }
  })
}
