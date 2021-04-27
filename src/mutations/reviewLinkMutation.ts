import { graphql } from 'react-relay'

import { ReviewLinkInput } from '__generated__/reviewLinkMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = ReviewLinkInput

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
