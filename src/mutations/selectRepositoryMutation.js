import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation selectRepositoryMutation(
    $input: SelectRepositoryInput!
  ) {
    selectRepository(input: $input) {
      viewer {
        ...AddForm_viewer
      }

      repository {
        id
        isPrivate
      }
    }
  }
`)
