import { GraphQLClient } from 'graphql-request'

const endpoint = 'http://localhost:3002/api/graphql'

export const graphQLClient = new GraphQLClient(endpoint, { fetch })
