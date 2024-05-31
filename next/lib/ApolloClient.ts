import { ApolloClient, HttpLink, InMemoryCache } from '@apollo/client'
import { registerApolloClient } from '@apollo/experimental-nextjs-app-support/rsc'

export const { getClient } = registerApolloClient(() => {
  const apiOrigin = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'
  const link = new HttpLink({
    uri: `${apiOrigin}/graphql`,
    fetchOptions: { cache: 'force-cache' },
  })

  return new ApolloClient({ cache: new InMemoryCache(), link })
})
