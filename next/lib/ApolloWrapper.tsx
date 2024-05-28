'use client'

import {
  ApolloLink,
  HttpLink,
} from '@apollo/client'
import {
  ApolloNextAppProvider,
  NextSSRApolloClient,
  NextSSRInMemoryCache,
  SSRMultipartLink,
} from '@apollo/experimental-nextjs-app-support/ssr'
import { loadErrorMessages, loadDevMessages } from '@apollo/client/dev'

if (process.env.NODE_ENV !== 'production') {
  // Adds messages only in a dev environment
  loadDevMessages()
  loadErrorMessages()
}

const apiOrigin = process.env.DIGRAPH_API_EXTERNAL_URL || 'http://localhost:8080'

function makeClient() {
  const httpLink = new HttpLink({
    uri: `${apiOrigin}/graphql`,
  })

  return new NextSSRApolloClient({
    cache: new NextSSRInMemoryCache(),
    link:
      typeof window === 'undefined'
        ? ApolloLink.from([
          new SSRMultipartLink({
            stripDefer: true,
          }),
          httpLink,
        ])
        : httpLink,
  })
}

export function ApolloWrapper({ children }: React.PropsWithChildren) {
  return (
    <ApolloNextAppProvider makeClient={makeClient}>
      {children}
    </ApolloNextAppProvider>
  )
}
