import { graphql } from 'react-relay'

export default graphql`
  mutation reviewLinkMutation(
    $input: ReviewLinkInput!
  ) {
    reviewLink(input: $input) {
      link {
        ...Review_link
      }
    }
  }
`
