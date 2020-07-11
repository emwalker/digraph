// @flow
import { type Node } from 'react'
import serialize from 'serialize-javascript'
import { renderToStringAsync } from 'react-async-ssr'

const getDeferScript = (src) => (src
  ? `<script defer src="${src}"></script>`
  : '')

const getStylesheet = (href) => (href
  ? `<link rel="stylesheet" type="text/css" href="${href}">`
  : '')

const joinArray = (fn, array) => (array
  ? array.map(fn).join('\n')
  : '')

const googleAnalytics = (gaId) => {
  if (!gaId) return ''

  return `
  <script async src="https://www.googletagmanager.com/gtag/js?id=${gaId}"></script>
  <script>
    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());
    gtag('config', '${gaId}');
  </script>
  `
}

const template = (vo) => `
<html>
  <head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0">
    ${googleAnalytics(vo.gaId)}
    <meta charset="utf-8">
    <meta http-equiv="Content-Language" content="en">
    <title>Digraph</title>
    <link rel="icon" type="image/x-icon" href="/static/images/favicon.ico">
    ${joinArray(getStylesheet, vo.vendorCSSBundle)}
    ${getStylesheet(vo.mainCSSBundle)}
  </head>

  <body>
    <div id="root"><div>${vo.root}</div></div>
    ${joinArray(getDeferScript, vo.vendorJSBundle)}
    ${getDeferScript(vo.mainJSBundle)}
    <script>
      window.__RELAY_PAYLOADS__ = ${vo.relayPayloads};
      window.__PRELOADED_STATE__ = ${vo.state};
    </script>
  </body>
</html>
`

const toString = (state) => JSON.stringify(state).replace(/</g, '\\u003c')

export default async (
  assets: Object, fetcher: Function, element: Node, preloadedState: Object,
): Promise<string> => {
  const vendor = assets[''] || {}
  const root = await renderToStringAsync(element)

  const html = template({
    gaId: process.env.DIGRAPH_GOOGLE_ANALYTICS_ID,
    mainCSSBundle: assets.client.css,
    mainJSBundle: assets.client.js,
    relayPayloads: serialize(fetcher, { isJSON: true }),
    root,
    vendorCSSBundle: vendor.css,
    vendorJSBundle: vendor.js,
    state: toString(preloadedState),
  })
  return Promise.resolve(html)
}
