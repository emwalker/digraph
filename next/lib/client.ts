import { GraphQLClient } from 'graphql-request'

const endpoint = 'http://0.0.0.0:3002/api/graphql'

export const graphQLClient = new GraphQLClient(endpoint, { fetch })
