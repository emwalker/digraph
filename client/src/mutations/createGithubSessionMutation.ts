import { graphql } from 'react-relay'

export default graphql`
  mutation createGithubSessionMutation(
    $input: CreateGithubSessionInput!
  ) {
    createGithubSession(input: $input) {
      alerts {
        id
        text
        type
      }

      userEdge {
        node {
          id
        }
      }

      sessionEdge {
        node {
          id
        }
      }
    }
  }
`
