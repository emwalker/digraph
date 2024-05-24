import { ReactNode } from 'react'
import serialize from 'serialize-javascript'
import { renderToStringAsync } from 'react-async-ssr'

import { FetcherBase } from '../FetcherBase'

const getDeferScript = (src: string) => (src
  ? `<script defer src="${src}"></script>`
  : '')

const getStylesheet = (href: string) => (href
  ? `<link rel="stylesheet" type="text/css" href="${href}">`
  : '')

const joinArray = (fn: (a: string) => string, array: string[] | null) => (array
  ? array.map(fn).join('\n')
  : '')

const googleAnalytics = (gaId: string | null) => {
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

type Variables = {
  gaId: string | null,
  root: string,
  vendorCSSBundle: string[] | null,
  vendorJSBundle: string[] | null,
  mainCSSBundle: string,
  mainJSBundle: string,
  state: string,
  relayPayloads: string,
}

const template = (vo: Variables) => `
<!DOCTYPE html>
<html>
  <head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0">
    ${googleAnalytics(vo.gaId)}
    <meta charset="utf-8">
    <meta http-equiv="Content-Language" content="en">
    <title>Digraph</title>
    <link rel="icon" href="/static/images/favicon.svg" type="image/svg+xml" sizes="any">
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

const toString = (state: Object) => JSON.stringify(state).replace(/</g, '\\u003c')

type Assets = {
  '': {
    css: string[],
    js: string[],
  } | undefined,
  client: { [key: string]: string },
}

export default async (
  assets: Assets, fetcher: FetcherBase, element: ReactNode, preloadedState: Object,
): Promise<string> => {
  const vendor = assets[''] || { css: null, js: null }
  const root = await renderToStringAsync(element)

  const html = template({
    gaId: process.env.DIGRAPH_GOOGLE_ANALYTICS_ID || null,
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
