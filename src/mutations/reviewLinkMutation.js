import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation reviewLinkMutation(
    $input: ReviewLinkInput!
  ) {
    reviewLink(input: $input) {
      link {
        ...Review_link
      }
    }
  }
`)
