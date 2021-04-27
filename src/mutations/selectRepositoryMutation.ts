import { graphql } from 'react-relay'

import { SelectRepositoryInput } from '__generated__/selectRepositoryMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = SelectRepositoryInput

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
