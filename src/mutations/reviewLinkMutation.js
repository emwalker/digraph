// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { ReviewLinkInput } from './__generated__/reviewLinkMutation.graphql'

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
