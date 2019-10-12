// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { SelectRepositoryInput } from './__generated__/selectRepositoryMutation.graphql'

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
