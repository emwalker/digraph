import express from 'express'
import { getFarceResult } from 'found/lib/server'
import ReactDOMServer from 'react-dom/server'
import serialize from 'serialize-javascript'
import webpack from 'webpack'
import webpackMiddleware from 'webpack-dev-middleware'
import path from 'path'

import webpackConfig from '../webpack.config'
import { ServerFetcher } from './fetcher'
import {
  createResolver,
  historyMiddlewares,
  render,
  routeConfig,
} from './router'

const PORT = 8080

const app = express()

app.use('/static', express.static(path.join(__dirname, 'static')))

app.use(
  webpackMiddleware(webpack(webpackConfig), {
    stats: { colors: true },
  }),
)

app.use(async (req, res) => {
  const fetcher = new ServerFetcher('http://localhost:5000/graphql')

  const { redirect, status, element } = await getFarceResult({
    url: req.url,
    historyMiddlewares,
    routeConfig,
    resolver: createResolver(fetcher),
    render,
  })

  if (redirect) {
    res.redirect(302, redirect.url)
    return
  }

  res.status(status).send(`
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Language" content="en">
  <title>Digraffe</title>
  <link rel="stylesheet" href="/style.css">
  <link rel="icon" type="image/x-icon" href="/static/images/favicon.ico">
  <link rel="stylesheet" href="https://primer.github.io/archive/docs.css">
  <link rel="stylesheet" href="https://unpkg.com/react-select@1.2.1/dist/react-select.css">
</head>

<body>
<div id="root">${ReactDOMServer.renderToString(element)}</div>

<script>
  window.__RELAY_PAYLOADS__ = ${serialize(fetcher, { isJSON: true })};
</script>
<script src="/bundle.js"></script>
</body>

</html>
  `)
})

app.listen(PORT, () => {
  // eslint-disable-next-line no-console
  console.log(`listening on port ${PORT}`)
})
