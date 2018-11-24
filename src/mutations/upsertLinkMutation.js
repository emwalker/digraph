import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'

export default defaultMutation(graphql`
  mutation upsertLinkMutation(
    $input: UpsertLinkInput!
  ) {
    upsertLink(input: $input) {
      linkEdge {
        node {
          ...Link_link
        }
      }
    }
  }
`)
